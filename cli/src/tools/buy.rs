use crate::database::Database;
use crate::error::*;

use eve_online_api::{EveClient, MarketOrder, RegionId, SystemId, TypeId};
use prettytable::{cell, row, Table};
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct BuyItemResult {
    pub item: String,
    pub price: f32,
    pub volume_remain: u32,
    pub system: String,
    pub region: String,
}

#[derive(Clone, Debug)]
pub struct BuyItemRawResult {
    pub type_id: TypeId,
    pub price: f32,
    pub volume_remain: u32,
    pub system_id: SystemId,
    pub region_id: RegionId,
}

pub struct BuyItem {
    database: Arc<Mutex<Database>>,
}

impl BuyItem {
    pub fn new(database: Arc<Mutex<Database>>) -> Self {
        Self { database }
    }

    pub async fn collect(
        &self,
        item: String,
        regions: Option<Vec<String>>,
        count: Option<usize>,
    ) -> Result<Vec<BuyItemResult>> {
        let raw_data = self.collect_raw(item, regions, count).await?;
        let mut results = Vec::with_capacity(raw_data.len());

        for raw in raw_data {
            let mut db = self.database.lock().unwrap();

            results.push(BuyItemResult {
                item: db.fetch_item(&raw.type_id).await?.name,
                price: raw.price,
                volume_remain: raw.volume_remain,
                system: db.fetch_system(&raw.system_id).await?.name,
                region: db.fetch_region(&raw.region_id).await?.name,
            })
        }

        Ok(results)
    }

    pub async fn collect_raw(
        &self,
        item: String,
        regions: Option<Vec<String>>,
        count: Option<usize>,
    ) -> Result<Vec<BuyItemRawResult>> {
        let count = count.unwrap_or(3);
        let regions = crate::resolve_regions(self.database.clone(), regions).await?;
        let item = crate::resolve_items(self.database.clone(), vec![item]).await?;
        let item = item.get(0).unwrap();

        let mut market_orders = self.fetch_market_orders(item.clone(), regions).await?;
        market_orders.sort_by(|x, y| x.price.partial_cmp(&y.price).unwrap_or(Ordering::Equal));

        let mut results = Vec::with_capacity(market_orders.len());
        for order in market_orders.into_iter().take(count) {
            let mut db = self.database.lock().unwrap();

            results.push(BuyItemRawResult {
                type_id: order.type_id,
                price: order.price,
                volume_remain: order.volume_remain,
                system_id: order.system_id,
                region_id: db.fetch_system_region(&order.system_id).await?,
            })
        }

        Ok(results)
    }

    pub async fn collect_and_print(
        &self,
        item: String,
        regions: Option<Vec<String>>,
        count: Option<usize>,
    ) -> Result<()> {
        let mut table = Table::new();
        table.add_row(row!["Item", "Price", "Total sells", "System", "Region"]);

        self.collect(item, regions, count)
            .await?
            .into_iter()
            .for_each(|x| {
                table.add_row(row![x.item, x.price, x.volume_remain, x.system, x.region]);
            });

        table.printstd();

        Ok(())
    }

    async fn fetch_market_orders(
        &self,
        type_id: TypeId,
        regions: Vec<RegionId>,
    ) -> Result<Vec<MarketOrder>> {
        let progress = crate::new_progress_bar();
        let mut market_orders = Vec::new();

        for region_id in regions {
            {
                let mut db = self.database.lock().unwrap();
                progress.set_message(&format!(
                    "Fetching market orders for {} in region {}",
                    db.fetch_item(&type_id).await?.name,
                    db.fetch_region(&region_id).await?.name
                ));
            }

            market_orders.extend(
                EveClient::default()
                    .fetch_market_orders_by_id(&region_id, &type_id, "sell")
                    .await?
                    .unwrap_or_default(),
            );

            {
                let mut db = self.database.lock().unwrap();
                progress.finish_with_message(&format!(
                    "Fetched market orders for region {}",
                    db.fetch_region(&region_id).await?.name
                ));
            }
        }

        Ok(market_orders)
    }
}
