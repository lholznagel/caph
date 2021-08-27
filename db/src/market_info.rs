use async_trait::*;
use caph_eve_data_wrapper::{TypeId, OrderId, LocationId, SolarSystemId};
use cachem::{Parse, Cache, Command, Get, Key, Set, Save};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::BufStream;
use tokio::net::TcpStream;
use tokio::sync::{RwLock, watch::Receiver};

type Id  = OrderId;
type Val = MarketInfoEntry;
type Typ = HashMap<Id, Val>;

#[derive(Clone)]
pub struct MarketInfoCache {
    cache: Arc<RwLock<Typ>>,
    cnc:   Receiver<Command>,
}

impl MarketInfoCache {
    pub fn new(cnc: Receiver<Command>) -> Self {
        Self {
            cache: Arc::new(RwLock::default()),
            cnc,
        }
    }

    #[cfg(test)]
    pub fn new_test(cache: Typ, cnc: Receiver<Command>) -> Self {
        Self {
            cache: Arc::new(RwLock::new(cache)),
            cnc
        }
    }
}

impl Into<Arc<dyn Cache>> for MarketInfoCache {
    fn into(self) -> Arc<dyn Cache> {
        Arc::new(self)
    }
}

#[async_trait]
impl Cache for MarketInfoCache {
    fn name(&self) -> String {
        "market_infos".into()
    }

    async fn handle(&self, cmd: Command, buf: &mut BufStream<TcpStream>) {
        match cmd {
            Command::Get => {
                let key = Id::read(buf).await.unwrap();
                let val = self.get(key, None).await;
                val.write(buf).await.unwrap();
            }
            Command::MGet => {
                let keys = Vec::<Id>::read(buf).await.unwrap();
                let vals = self.mget(keys, None).await;
                vals.write(buf).await.unwrap();
            }
            Command::Set => {
                let key = Id::read(buf).await.unwrap();
                let val = Val::read(buf).await.unwrap();
                self.set(key, val).await;
                self.save().await;
                0u8.write(buf).await.unwrap();
            }
            Command::MSet => {
                let vals = HashMap::<Id, Val>::read(buf).await.unwrap();
                self.mset(vals).await;
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
impl Get for MarketInfoCache {
    type Id  =   Id;
    type Res =   Val;
    type Param = ();

    async fn get(&self, id: Self::Id, _: Option<Self::Param>) -> Option<Self::Res> {
        self
            .cache
            .read()
            .await
            .get(&id)
            .cloned()
    }
}

#[async_trait]
impl Set for MarketInfoCache {
    type Id  = Id;
    type Val = Val;

    async fn set(&self, id: Self::Id, val: Self::Val) {
        self
            .cache
            .write()
            .await
            .insert(id, val);
    }
}

#[async_trait]
impl Key for MarketInfoCache {
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
impl Save for MarketInfoCache {
    type Typ = Typ;

    fn file(&self) -> &str {
        "./db/market_infos.cachem"
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
pub struct MarketInfoEntry {
    /// Timestamp in seconds, when this order was placed
    pub issued:       u64,
    /// Timestamp in seconds, when this order expires
    pub expire:       u64,
    pub order_id:     OrderId,
    pub location_id:  LocationId,
    pub system_id:    SolarSystemId,
    pub type_id:      TypeId,
    pub volume_total: u32,
    pub price:        f32,
    /// true  -> buy
    /// false -> sell
    pub is_buy_order: bool,
}

impl MarketInfoEntry {
    pub fn new(
        issued:       u64,
        expire:       u64,
        order_id:     OrderId,
        location_id:  LocationId,
        system_id:    SolarSystemId,
        type_id:      TypeId,
        volume_total: u32,
        price:        f32,
        is_buy_order: bool,
    ) -> Self {
        Self {
            issued,
            expire,
            order_id,
            location_id,
            system_id,
            type_id,
            volume_total,
            price,
            is_buy_order,
        }
    }
}

