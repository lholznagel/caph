use crate::error::CollectorError;
use crate::time::previous_30_minute;

use cachem::v2::ConnectionPool;
use caph_db::*;
use caph_eve_data_wrapper::{EveDataWrapper, IndustryService, MarketService, SolarSystemId, SystemService, TypeId};
use chrono::{DateTime, Utc};
use futures::stream::{FuturesUnordered, StreamExt};
use std::collections::HashMap;

pub struct Market {
    eve:  EveDataWrapper,
    pool: ConnectionPool,
}

impl Market {
    pub fn new(eve: EveDataWrapper, pool: ConnectionPool) -> Self {
        Self {
            eve,
            pool
        }
    }

    /// Runs a task in the background that periodically collects all market
    /// entries from all markets and writes them into the database.
    pub async fn task(&mut self) -> Result<(), CollectorError> {
        log::info!("Loading eve services");
        let market_service  = self.eve.market().await?;
        let system_service  = self.eve.systems().await?;
        let indutry_service = self.eve.industry().await?;
        log::info!("Services loaded");

        let _ = tokio::join! {
            self.market_data(market_service.clone(), system_service),
            self.market_price(market_service.clone()),
            self.industry_cost(indutry_service)
        };

        Ok(())
    }

    async fn market_data(
        &self,
        market_service: MarketService,
        system_service: SystemService,
    ) -> Result<(), CollectorError> {

        let timestamp = previous_30_minute(Utc::now().timestamp() as u64)? * 1_000;

        let mut requests = FuturesUnordered::new();
        let regions = system_service.region_ids();

        for region in regions {
            requests.push(market_service.orders(*region));
        }

        let mut entries = Vec::new();
        while let Some(return_val) = requests.next().await {
            if let Ok(r) = return_val {
                entries.extend(r);
            }
        }

        let mut con = self.pool.acquire().await?;

        let mut market_infos = HashMap::new();
        let mut market_orders = HashMap::new();

        for entry in entries.iter() {
            let issued = entry.issued.parse::<DateTime<Utc>>()?;
            let expire = issued.checked_add_signed(
                chrono::Duration::days(entry.duration as i64)
            ).ok_or(CollectorError::ChronoError)?;

            let market_info = MarketInfoEntry {
                issued:       issued.timestamp() as u64 * 1000,
                expire:       expire.timestamp() as u64 * 100,
                order_id:     entry.order_id.into(),
                location_id:  entry.location_id.into(),
                system_id:    entry.system_id.into(),
                type_id:      entry.type_id.into(),
                volume_total: entry.volume_total,
                price:        entry.price,
                is_buy_order: entry.is_buy_order,
            };
            market_infos.insert(entry.order_id, market_info);
        }

        for entry in entries {
            let market_order = MarketOrderEntry {
                order_id:      entry.order_id.into(),
                timestamp,
                volume_remain: entry.volume_remain,
                type_id:       entry.type_id.into(),
            };
            market_orders
                .entry(entry.type_id)
                .and_modify(|x: &mut Vec<MarketOrderEntry>| { x.push(market_order.clone()) })
                .or_insert(vec![market_order]);
        }

        if !market_infos.is_empty() {
            con.mset(CacheName::MarketInfo, market_infos).await.unwrap();
            con.mset(CacheName::MarketOrder, market_orders).await.unwrap();
        }

        Ok(())
    }

    async fn market_price(&self, market_service: MarketService) -> Result<(), CollectorError> {
        let mut con = self.pool.acquire().await?;

        let prices = market_service
            .prices()
            .await
            .unwrap_or_default()
            .into_iter()
            .map(MarketPriceEntry::from)
            .map(|x| (x.type_id, x))
            .collect::<HashMap<TypeId, MarketPriceEntry>>();
        con.mset(CacheName::MarketPrice, prices).await.unwrap();
        Ok(())
    }

    async fn industry_cost(&self, industry_service: IndustryService) -> Result<(), CollectorError> {
        let mut con = self.pool.acquire().await?;
        let cost = industry_service
            .systems()
            .await
            .unwrap_or_default()
            .into_iter()
            .map(IndustryCostEntry::from)
            .map(|x| (x.solar_system_id, x))
            .collect::<HashMap<SolarSystemId, IndustryCostEntry>>();
        con.mset(CacheName::IndustryCost, cost).await.unwrap();
        Ok(())
    }
}
