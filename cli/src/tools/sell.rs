use crate::database::Database;
use crate::error::*;
use crate::SellOreCli;

use clap::Clap;
use caph_eve_online_api::{AttributeId, EveClient, MarketOrder, RegionId, SystemId, TypeId};
use num_format::{Locale, ToFormattedString};
use prettytable::{cell, row, Cell, Row, Table};
use serde_json::json;
use serde::Deserialize;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct SellItemResult {
    pub item: String,
    pub density: f32,
    pub entries: Vec<SellItemEntry>,
}

#[derive(Clone, Debug)]
pub struct SellItemEntry {
    pub price: f32,
    pub volume_remain: u32,
    pub system: String,
    pub system_sec: String,
    pub region: String,
}

#[derive(Clone, Debug)]
pub struct SellItemRawResult {
    pub type_id: TypeId,
    pub density: f32,
    pub entries: Vec<SellItemRawEntry>,
}

#[derive(Clone, Debug)]
pub struct SellItemRawEntry {
    pub price: f32,
    pub volume_remain: u32,
    pub system_id: SystemId,
    pub region_id: RegionId,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct MarketCacheEntry {
    pub is_buy_order: bool,
    pub location_id: u64,
    pub price: f32,
    pub system_id: SystemId,
    pub type_id: TypeId,
    pub volume_remain: u32,
}

pub struct SellItem {
    database: Arc<Mutex<Database>>,
}

impl SellItem {
    pub fn new(database: Arc<Mutex<Database>>) -> Self {
        Self { database }
    }

    pub async fn collect(
        &self,
        items: Vec<String>,
        count: usize,
        regions: Option<Vec<String>>,
    ) -> Result<Vec<SellItemResult>> {
        let raw_data = self.collect_raw(items, count).await?;
        let mut results = Vec::with_capacity(raw_data.len());

        for raw in raw_data {
            let mut db = self.database.lock().unwrap();
            let mut entries = Vec::with_capacity(3);

            for entry in &raw.entries {
                let sec = db
                    .fetch_system(&entry.system_id)
                    .await?
                    .security_status
                    .to_string();
                let system_sec = if sec.len() > 3 {
                    sec[..3].to_string()
                } else {
                    sec
                };

                entries.push(SellItemEntry {
                    price: entry.price,
                    volume_remain: entry.volume_remain,
                    system: db.fetch_system(&entry.system_id).await?.name,
                    system_sec,
                    region: db.fetch_region(&entry.region_id).await?.name,
                });
            }

            results.push(SellItemResult {
                item: db.fetch_item(&raw.type_id).await?.name,
                density: raw.density,
                entries,
            });
        }

        Ok(results)
    }

    pub async fn collect_raw(
        &self,
        items: Vec<String>,
        count: usize,
    ) -> Result<Vec<SellItemRawResult>> {
        let items = crate::resolve_items(self.database.clone(), items).await?;

        let mut market_orders = self.fetch_market_orders(items.clone()).await?;
        market_orders.sort_by(|x, y| y.price.partial_cmp(&x.price).unwrap_or(Ordering::Equal));

        let progress = crate::new_progress_bar();
        progress.set_message("Preparing data");
        let orders = items
            .into_iter()
            .map(|type_id| {
                market_orders
                    .clone()
                    .into_iter()
                    .filter(|x| x.type_id == type_id)
                    .take(count)
                    .collect::<Vec<MarketCacheEntry>>()
            })
            .map(|mut x| {
                x.sort_by(|x, y| y.price.partial_cmp(&x.price).unwrap_or(Ordering::Equal));
                x
            })
            .collect::<Vec<Vec<MarketCacheEntry>>>();

        let mut results = Vec::with_capacity(orders.len());
        for order in orders {
            let mut db = self.database.lock().unwrap();
            let mut entries = Vec::with_capacity(count);

            for entry in &order {
                entries.push(SellItemRawEntry {
                    price: entry.price,
                    volume_remain: entry.volume_remain,
                    system_id: entry.system_id,
                    region_id: db.fetch_system_region(&entry.system_id).await?,
                });
            }

            let item = EveClient::default()
                .fetch_type(order.get(0).unwrap().type_id)
                .await?
                .unwrap();
            let density = item
                .dogma_attributes
                .unwrap_or_default()
                .into_iter()
                // Attribute 161 -> density
                .find(|x| x.attribute_id == AttributeId(161))
                .map(|x| x.value)
                .unwrap_or_default();

            results.push(SellItemRawResult {
                type_id: item.type_id,
                density,
                entries,
            });
        }
        progress.finish_with_message("Data ready");

        Ok(results)
    }

    pub async fn collect_and_print(
        &self,
        type_ids: Vec<String>,
        count: usize,
        regions: Option<Vec<String>>,
    ) -> Result<()> {
        let mut table = Table::new();
        table.add_row(row![
            "Item",
            "Price",
            "Total orders",
            "Price / Density",
            "System",
            "Sec",
            "Region"
        ]);

        self.collect(type_ids, count, regions)
            .await?
            .into_iter()
            .for_each(|x| {
                let mut entry_price = String::new();
                let mut entry_price_density = String::new();
                let mut entry_volume_remain = String::new();
                let mut entry_system = String::new();
                let mut entry_system_sec = String::new();
                let mut entry_region = String::new();

                for entry in x.entries {
                    entry_price.push_str(&entry.price.to_string());
                    entry_price.push_str("\n");

                    entry_price_density
                        .push_str(&((entry.price / x.density).round() as usize).to_string());
                    entry_price_density.push_str("\n");

                    entry_volume_remain
                        .push_str(&entry.volume_remain.to_formatted_string(&Locale::de));
                    entry_volume_remain.push_str("\n");

                    entry_system.push_str(&entry.system.to_string());
                    entry_system.push_str("\n");

                    entry_system_sec.push_str(&entry.system_sec.to_string());
                    entry_system_sec.push_str("\n");

                    entry_region.push_str(&entry.region.to_string());
                    entry_region.push_str("\n");
                }

                table.add_row(Row::new(vec![
                    Cell::new(&x.item),
                    Cell::new(&entry_price),
                    Cell::new(&entry_volume_remain),
                    Cell::new(&entry_price_density),
                    Cell::new(&entry_system),
                    Cell::new(&entry_system_sec),
                    Cell::new(&entry_region),
                ]));
            });

        table.printstd();

        Ok(())
    }

    async fn fetch_market_orders(
        &self,
        type_ids: Vec<TypeId>,
    ) -> Result<Vec<MarketCacheEntry>> {
        surf::client()
            .post("http://localhost:9000/api/market")
            .body(json!({
                "ids": type_ids,
                "onlyBuyOrders": true
            }))
            .send()
            .await
            .unwrap()
            .body_json::<Vec<MarketCacheEntry>>()
            .await
            .map_err(EveError::SurfError)
    }
}

#[derive(Clap)]
pub struct SellCli {
    /// Number of entries shown per item
    #[clap(long, short, default_value = "3")]
    pub entries: usize,
    /// Items to check get the currently best selling price
    #[clap(long, short, multiple = true)]
    pub items: Vec<String>,
    /// Regions to check for the best price, per default it will use all non null-sec regions
    #[clap(long, short, multiple = true)]
    pub regions: Option<Vec<String>>,
    // future filters
    // region min sec
    // min sell price
    #[clap(subcommand)]
    pub subcmd: Option<SellSubcommand>,
}

#[derive(Clap)]
pub enum SellSubcommand {
    Ore(SellOreCli),
}
