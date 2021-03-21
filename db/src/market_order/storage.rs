use crate::MarketItemOrderId;

use super::{MarketOrderCache, MarketOrderSaveEntry};

use async_trait::async_trait;
use cachem::{CachemError, Parse, Storage};
use std::collections::HashMap;
use std::time::Instant;
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite};

const METRIC_STORAGE_LOAD: &'static str = "storage::market_order::load";
const METRIC_STORAGE_SAVE: &'static str = "storage::market_order::save";

#[async_trait]
impl Storage for MarketOrderCache {
    fn file() -> &'static str {
        "./db/storage/market_orders.cachem"
    }

    async fn load<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncBufRead + AsyncRead + Send + Unpin {

        let timer = Instant::now();

        if let Ok(entries) = SaveMarketHistory::read(buf).await {
            let mut map = HashMap::new();

            for entry in entries.0 {
                let mut orders = HashMap::new();
                for x in entry.entries {
                    orders
                        .entry(x.order_id)
                        .and_modify(|y: &mut Vec<MarketItemOrderId>| y.push(x.clone()))
                        .or_insert(vec![x]);
                }
                map.insert(entry.item_id, orders);
            }

            *self.history.write().await = map;
        }

        self.metrix.send_time(METRIC_STORAGE_LOAD, timer).await;
        Ok(())
    }

    async fn save<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncWrite + Send + Unpin {

        let timer = Instant::now();
        let data_copy = self.history.read().await;

        let mut save_entries = Vec::with_capacity(data_copy.len());
        for (item, history) in self.history.read().await.iter() {
            let mut values = Vec::new();
            for (_, e) in history {
                values.extend(e.clone());
            }

            save_entries.push(MarketOrderSaveEntry {
                item_id: *item,
                entries: values,
            });
        }

        SaveMarketHistory(save_entries)
            .write(buf)
            .await?;

        self.metrix.send_time(METRIC_STORAGE_SAVE, timer).await;
        Ok(())
    }
}

#[derive(Debug, Parse)]
pub struct SaveMarketHistory(pub Vec<MarketOrderSaveEntry>);
