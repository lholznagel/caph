use async_trait::*;
use caph_eve_data_wrapper::{TypeId, OrderId};
use cachem::{Parse, Cache, Command, Get, Key, Set, Save};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::BufStream;
use tokio::net::TcpStream;
use tokio::sync::{RwLock, watch::Receiver};

use crate::MarketInfoCache;

type Id  = TypeId;
type Val = MarketOrder;
type Typ = HashMap<Id, HashMap<OrderId, Vec<Val>>>;

pub struct MarketOrderCache {
    cache: RwLock<Typ>,
    cnc:   Receiver<Command>,

    market_info: MarketInfoCache,
}

impl MarketOrderCache {
    pub fn new(
        cnc: Receiver<Command>,

        market_info: MarketInfoCache,
    ) -> Self {
        Self {
            cache: RwLock::default(),
            cnc,

            market_info
        }
    }
}

impl Into<Arc<dyn Cache>> for MarketOrderCache {
    fn into(self) -> Arc<dyn Cache> {
        Arc::new(self)
    }
}

#[async_trait]
impl Cache for MarketOrderCache {
    fn name(&self) -> String {
        "market_orders".into()
    }

    async fn handle(&self, cmd: Command, buf: &mut BufStream<TcpStream>) {
        match cmd {
            Command::Get => {
                let key = Id::read(buf).await.unwrap();
                let params = Option::<MarketOrderRequest>::read(buf).await.unwrap();
                let val = self.get(key, params).await;
                val.write(buf).await.unwrap();
            }
            Command::Set => {
                let key = Id::read(buf).await.unwrap();
                let val = Vec::<MarketOrderEntry>::read(buf).await.unwrap();
                self.set(key, val).await;
                self.save().await;
                0u8.write(buf).await.unwrap();
            }
            Command::MSet => {
                let data = HashMap::<Id, Vec<MarketOrderEntry>>::read(buf).await.unwrap();
                self.mset(data).await;
                self.save().await;
                0u8.write(buf).await.unwrap();
            }
            Command::Keys => {
                self.keys().await.write(buf).await.unwrap();
            }
            _ => {
                log::error!("Invalid cmd {:?}", cmd);
            }
        }
    }

    async fn cnc_listener(&self) {
        let mut cnc_copy = self.cnc.clone();
        loop {
            cnc_copy.changed().await.unwrap();
            let cmd = *cnc_copy.borrow();

            match cmd {
                Command::Save => { self.save().await; },
                _ => { log::warn!("Invalid cmd send over cnc: {:?}", cmd); }
            }
        }
    }
}

#[async_trait]
impl Get for MarketOrderCache {
    type Id    = Id;
    type Res   = Vec<MarketOrderResponse>;
    type Param = MarketOrderRequest;

    async fn get(&self, id: Self::Id, params: Option<Self::Param>) -> Option<Self::Res> {
        let ts_start = params.clone().unwrap_or_default().start;
        let ts_stop  = params.unwrap_or_default().end;

        // Get all item entries that are newer than the given start timestamp
        let historic = self
            .cache
            .read()
            .await;

        let mut items = Vec::new();
        let entries = if let Some(e) = historic.get(&id) { e.clone() } else { HashMap::new() };
        for (_, entries) in entries.iter() {
            let mut index = 0;
            loop {
                if let Some(x) = entries.get(index) {
                    if x.timestamp < ts_start {
                        index += 1;
                    } else {
                        break;
                    }
                } else {
                    // our timeslot is one back
                    index -= 1;
                    break;
                }
            }
            items.extend(entries[index..].to_vec());
        }

        // Get a list of all order ids and make sure that every id only exist once
        // This is needed to detect missing entries
        let mut unique_order_ids = items.iter().map(|x| x.order_id).collect::<Vec<_>>();
        unique_order_ids.sort();
        unique_order_ids.dedup();

        // Collect the expire date for all orders
        let order_id_expire = self.market_info
            .mget(unique_order_ids.clone(), None)
            .await
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap())
            .map(|x| (x.order_id, x.expire))
            .collect::<HashMap<_, _>>();

        // Stores all already seen values.
        // Used to fill up missing values
        let mut last_values: HashMap<u64, u32> = HashMap::new();
        let mut ret = HashMap::new();

        let mut ts_current = ts_start;
        while ts_current <= ts_stop {
            // Gets all items that have the current timestamp
            let mut items_filter = items
                .iter()
                .filter(|x| x.timestamp == ts_current)
                .map(|x| (x.order_id, x.volume))
                .collect::<Vec<(OrderId, u32)>>();
            // Loop over all found items and insert them as there last value
            for (order, volume) in items_filter.iter() {
                last_values.insert(**order, *volume);
            }

            // If the list of items is smaller than the list of unique order ids
            // we need to search which are missing and fill them in
            if items_filter.len() < unique_order_ids.len() {
                // Filter all order ids that we already have
                let item_orders = items_filter
                    .iter()
                    .map(|(order, _)| *order)
                    .collect::<Vec<_>>();
                // Find out what order ids are missing
                let missing = unique_order_ids
                    .iter()
                    .filter(|x| !item_orders.contains(x))
                    .collect::<Vec<_>>();
                for order_id in missing {
                    // Check if we have an old value in the map and insert it
                    let last_value = last_values.get(&order_id);

                    let volume = if let Some(x) = last_value {
                        *x
                    } else {
                        // We don´t have an old value, so we look in the history
                        // if there is an older entry
                        let items = self.cache
                            .read()
                            .await;
                        let mut items = items
                            .get(&id)
                            .unwrap()
                            .get(order_id)
                            .unwrap()
                            .into_iter()
                            .filter(|x| x.timestamp < ts_current)
                            .collect::<Vec<_>>();
                        if items.len() > 0 {
                            // Sort the items by timestamp
                            items.sort_by(|a, b| 
                                          b.timestamp.partial_cmp(&a.timestamp)
                                                    .unwrap_or(std::cmp::Ordering::Equal));
                            // Take the first item and return its volume
                            // We check if the result has more than 0 elements
                            // so the unwrap is save
                            items.first().unwrap().volume
                        } else {
                            // If we don´t have an old value and there is no
                            // historic value, return 0
                            0
                        }
                    };

                    // Check if the current order is expired, if so, don´t
                    // add it to the result.
                    if let Some(x) = order_id_expire.get(order_id) {
                        if ts_current <= *x {
                            // Make sure that we insert the new last value in the map
                            // for later lookups
                            last_values.insert(**order_id, volume);
                            // Push the value into the result
                            items_filter.push((*order_id, volume));
                        }
                    }
                }
            }

            // Sort the items by there order id to make sure that those
            // are always in order
            items_filter.sort_by(|(a, _), (b, _)| 
                                 a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

            // Add the items with the current ts into the return map
            ret.insert(ts_current, items_filter);

            // Increase the timestamp to the next 30 minute mark in milliseconds
            ts_current += 1_800 * 1_000;
        }

        let mut response = Vec::with_capacity(ret.len());
        for (ts, data) in ret {
            let mut entries = Vec::with_capacity(data.len());
            for (order, volume) in data {
                entries.push(MarketOrderResponseEntry {
                    order_id: order,
                    volume
                });
            }
            response.push(MarketOrderResponse {
                timestamp: ts,
                entries
            });
        }
        response.sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap_or(std::cmp::Ordering::Equal));

        Some(response)
    }
}

#[async_trait]
impl Set for MarketOrderCache {
    type Id  = Id;
    type Val = Vec<MarketOrderEntry>;

    async fn set(&self, _id: Self::Id, val: Self::Val) {
        for entry in val {
            self
                .cache
                .write()
                .await
                .entry(entry.type_id)
                .and_modify(|x| {
                    if let Some(y) = x.get_mut(&entry.order_id) {
                        if y.last().unwrap().volume != entry.volume_remain {
                            y.push(
                                MarketOrder {
                                    timestamp: entry.timestamp,
                                    order_id: entry.order_id,
                                    volume: entry.volume_remain,
                                }
                            )
                        }
                    } else {
                        x.insert(
                            entry.order_id,
                            vec![
                                MarketOrder {
                                    timestamp: entry.timestamp,
                                    order_id: entry.order_id,
                                    volume: entry.volume_remain,
                                }
                            ]
                        );
                    }
                })
                .or_insert({
                    let mut map = HashMap::new();
                    map.insert(
                        entry.order_id,
                        vec![
                            MarketOrder {
                                timestamp: entry.timestamp,
                                order_id: entry.order_id,
                                volume: entry.volume_remain,
                            }
                        ]
                    );
                    map
                });
        }
    }
}

#[async_trait]
impl Key for MarketOrderCache {
    type Id = Id;

    async fn keys(&self) -> Vec<Self::Id> {
        self
            .cache
            .read()
            .await
            .keys()
            .map(|x| *x)
            .collect::<Vec<_>>()
    }
}

#[async_trait]
impl Save for MarketOrderCache {
    type Typ = Typ;

    fn file(&self) -> &str {
        "./db/market_orders.cachem"
    }

    async fn read(&self) -> Self::Typ {
        self.cache.read().await.clone()
    }

    async fn write(&self, data: Self::Typ) {
        *self.cache.write().await = data;
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct MarketOrderEntry {
    pub order_id:      OrderId,
    pub timestamp:     u64,
    pub volume_remain: u32,
    pub type_id:       TypeId,
}

impl MarketOrderEntry {
    pub fn new(
        order_id:      OrderId,
        timestamp:     u64,
        volume_remain: u32,
        type_id:       TypeId,
    ) -> Self {
        Self {
            order_id,
            timestamp,
            volume_remain,
            type_id,
        }
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct MarketOrder {
    pub order_id:  OrderId,
    pub timestamp: u64,
    pub volume:    u32,
}

impl MarketOrder {
    pub fn new(
        order_id:  OrderId,
        timestamp: u64,
        volume:    u32,
    ) -> Self {
        Self {
            order_id,
            timestamp,
            volume,
        }
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct MarketOrderResponse {
    pub timestamp: u64,
    pub entries:   Vec<MarketOrderResponseEntry>,
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct MarketOrderResponseEntry {
    pub order_id: OrderId,
    pub volume:   u32,
}

#[derive(Clone, Debug, Default, Parse)]
pub struct MarketOrderRequest {
    pub start: u64,
    pub end:   u64,
}

#[cfg(test)]
mod tests_fetch_market_orders {
    use crate::MarketInfoEntry;

    use super::*;

    use tokio::sync::{RwLock, watch};

    // Timeslots that don´t contain a change should be the same as the last known
    // timeslot
    #[tokio::test]
    async fn one_value_three_timeslots() {
        let mut history = HashMap::new();
        let mut orders = HashMap::new();
        orders.insert(0u64.into(), vec![
            MarketOrder {
                timestamp: 0 * 1800 * 1000,
                order_id: 0u64.into(),
                volume: 100u32,
            }
        ]);
        history.insert(0u32.into(), orders);

        let order_info = MarketInfoEntry {
            order_id: 0u64.into(),
            type_id: 0u32.into(),
            price: 100f32,
            issued: 0 * 1800 * 1000,
            expire: 100 * 1800 * 1000,
            system_id: 0u32.into(),
            location_id: 0u64.into(),
            volume_total: 100,
            is_buy_order: true
        };
        let mut order_map = HashMap::new();
        order_map.insert(0u64.into(), order_info);

        let (_, rx) = watch::channel(Command::Ping);
        let market_info = MarketInfoCache::new_test(order_map, rx.clone());
        let cache = MarketOrderCache {
            cache:        RwLock::new(history),
            cnc:          rx,
            market_info,
        };

        let req = MarketOrderRequest { start: 0u64, end: 2 * 1800 * 1000 };
        let res = cache.get(0u32.into(), Some(req)).await.unwrap();
        dbg!(&res);
        assert_eq!(res.len(), 3);
        for x in res.iter() {
            assert_eq!(x.entries.len(), 1);
        }
    }

    // A order that is before the requested time, that has some volume but doesn´t
    // have any changes should still be in the result because its an active order.
    #[tokio::test]
    async fn value_before_requested_timeslot() {
        let mut history = HashMap::new();
        let mut orders = HashMap::new();
        orders.insert(0u64.into(), vec![
            MarketOrder {
                timestamp: 0 * 1800 * 1000,
                order_id: 0u64.into(),
                volume: 100u32,
            },
            MarketOrder {
                timestamp: 2 * 1800 * 1000,
                order_id: 0u64.into(),
                volume: 99u32,
            }
        ]);
        orders.insert(1u64.into(), vec![
            MarketOrder {
                timestamp: 0 * 1800 * 1000,
                order_id: 1u64.into(),
                volume: 50u32,
            }
        ]);
        history.insert(0u32.into(), orders);

        let order_info_0 = MarketInfoEntry {
            order_id: 0u64.into(),
            type_id: 0u32.into(),
            price: 100f32,
            issued: 0 * 1800 * 1000,
            expire: 100 * 1800 * 1000,
            system_id: 0u32.into(),
            location_id: 0u64.into(),
            volume_total: 100,
            is_buy_order: true
        };
        let order_info_1 = MarketInfoEntry {
            order_id: 1u64.into(),
            type_id: 0u32.into(),
            price: 100f32,
            issued: 0 * 1800 * 1000,
            expire: 100 * 1800 * 1000,
            system_id: 0u32.into(),
            location_id: 0u64.into(),
            volume_total: 100,
            is_buy_order: true
        };
        let mut order_map = HashMap::new();
        order_map.insert(0u64.into(), order_info_0);
        order_map.insert(1u64.into(), order_info_1);

        let (_, rx) = watch::channel(Command::Ping);
        let market_info = MarketInfoCache::new_test(order_map, rx.clone());
        let cache = MarketOrderCache {
            cache: RwLock::new(history),
            cnc: rx,
            market_info,
        };

        let req = MarketOrderRequest { start: 1 * 1800 * 1000, end: 3 * 1800 * 1000 };
        let res = cache.get(0u32.into(), Some(req)).await.unwrap();
        assert_eq!(res.len(), 3);
        for x in res.iter() {
            assert_eq!(x.entries.len(), 2);
        }
    }

    // Orders that are expired should not be added
    #[tokio::test]
    async fn one_value_expired_after_thee_timeslots() {
        let mut history = HashMap::new();
        let mut orders = HashMap::new();
        orders.insert(0u64.into(), vec![
            MarketOrder {
                timestamp: 0 * 1800 * 1000,
                order_id: 0u64.into(),
                volume: 100u32,
            }
        ]);
        history.insert(0u32.into(), orders);

        let order_info = MarketInfoEntry {
            order_id: 0u64.into(),
            type_id: 0u32.into(),
            price: 100f32,
            issued: 0 * 1800 * 1000,
            expire: 3 * 1800 * 1000,
            system_id: 0u32.into(),
            location_id: 0u64.into(),
            volume_total: 100,
            is_buy_order: true
        };
        let mut order_map = HashMap::new();
        order_map.insert(0u64.into(), order_info);

        let (_, rx) = watch::channel(Command::Ping);
        let market_info = MarketInfoCache::new_test(order_map, rx.clone());
        let cache = MarketOrderCache {
            cache: RwLock::new(history),
            cnc: rx,
            market_info,
        };

        let req = MarketOrderRequest { start: 0u64, end: 4 * 1800 * 1000 };
        let res = cache.get(0u32.into(), Some(req)).await.unwrap();
        assert_eq!(res.len(), 5);
        for x in res.iter().take(4) {
            assert_eq!(x.entries.len(), 1);
        }
        assert_eq!(res.iter().last().unwrap().entries.len(), 0);
    }

    // Takes the same timestamp for start and stop.
    // It is expected that all active orders for the given item are returned.
    #[tokio::test]
    async fn live_value() {
        let mut history = HashMap::new();
        let mut orders = HashMap::new();
        orders.insert(0u64.into(), vec![
            MarketOrder {
                timestamp: 0 * 1800 * 1000,
                order_id: 0u64.into(),
                volume: 100u32,
            },
            MarketOrder {
                timestamp: 2 * 1800 * 1000,
                order_id: 0u64.into(),
                volume: 99u32,
            }
        ]);
        orders.insert(1u64.into(), vec![
            MarketOrder {
                timestamp: 1 * 1800 * 1000,
                order_id: 1u64.into(),
                volume: 50u32,
            }
        ]);
        history.insert(0u32.into(), orders);

        let order_info_0 = MarketInfoEntry {
            order_id: 0u64.into(),
            type_id: 0u32.into(),
            price: 100f32,
            issued: 0 * 1800 * 1000,
            expire: 100 * 1800 * 1000,
            system_id: 0u32.into(),
            location_id: 0u64.into(),
            volume_total: 100,
            is_buy_order: true
        };
        let order_info_1 = MarketInfoEntry {
            order_id: 1u64.into(),
            type_id: 0u32.into(),
            price: 100f32,
            issued: 0 * 1800 * 1000,
            expire: 100 * 1800 * 1000,
            system_id: 0u32.into(),
            location_id: 0u64.into(),
            volume_total: 100,
            is_buy_order: true
        };
        let mut order_map = HashMap::new();
        order_map.insert(0u64.into(), order_info_0);
        order_map.insert(1u64.into(), order_info_1);

        let (_, rx) = watch::channel(Command::Ping);
        let market_info = MarketInfoCache::new_test(order_map, rx.clone());
        let cache = MarketOrderCache {
            cache: RwLock::new(history),
            cnc: rx,
            market_info,
        };

        let req = MarketOrderRequest { start: 3 * 1800 * 1000, end: 3 * 1800 * 1000 };
        let res = cache.get(0u32.into(), Some(req)).await.unwrap();
        assert_eq!(res.len(), 1);
        for x in res.iter() {
            assert_eq!(x.entries.len(), 2);
        }
        assert_eq!(res.get(0).unwrap().entries.get(0).unwrap().volume, 99);
        assert_eq!(res.get(0).unwrap().entries.get(1).unwrap().volume, 50);
    }
}
