use super::{MarketOrderCache, MarketOrderSaveEntry};

use async_trait::async_trait;
use cachem::{CachemError, Parse, Storage};
use std::collections::HashMap;
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite};
use tokio::sync::RwLock;

#[async_trait]
impl Storage for MarketOrderCache {
    fn file() -> &'static str {
        "./db/storage/market_orders.cachem"
    }

    async fn load<B>(buf: &mut B) -> Result<Self, CachemError>
        where B: AsyncBufRead + AsyncRead + Send + Unpin {

        if let Ok(entries) = SaveMarketHistory::read(buf).await {
            let mut map = HashMap::new();

            for entry in entries.0 {
                let mut entries = Vec::with_capacity(entry.entries.len());
                for x in entry.entries {
                    entries.push(x);
                }
                map.insert(entry.item_id, entries);
            }

            Ok(MarketOrderCache {
                current: RwLock::new(HashMap::new()),
                history: RwLock::new(map),
            })
        } else {
            Ok(MarketOrderCache::default())
        }
    }

    async fn save<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncWrite + Send + Unpin {

        let data_copy = self.history.read().await;

        let mut save_entries = Vec::with_capacity(data_copy.len());
        for (item, history) in self.history.read().await.iter() {
            save_entries.push(MarketOrderSaveEntry {
                item_id: *item,
                entries: history.clone(),
            });
        }

        SaveMarketHistory(save_entries)
            .write(buf)
            .await
            .map(drop)
    }
}

#[derive(Debug, Parse)]
pub struct SaveMarketHistory(pub Vec<MarketOrderSaveEntry>);
