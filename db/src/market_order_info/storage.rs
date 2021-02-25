use super::{MarketOrderInfoCache, MarketOrderInfoEntry};

use async_trait::async_trait;
use cachem::{CachemError, Parse, Storage};
use std::collections::HashMap;
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite};

#[async_trait]
impl Storage for MarketOrderInfoCache {
    fn file() -> &'static str {
        "./db/storage/market_order_infos.cachem"
    }

    async fn load<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncBufRead + AsyncRead + Send + Unpin {

        if let Ok(entries) = SaveMarketInfos::read(buf).await {
            let mut map = HashMap::with_capacity(entries.0.len());
            for entry in entries.0 {
                map.insert(entry.order_id, entry);
            }

            *self.0.write().await = map;
        }
        Ok(())
    }

    async fn save<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncWrite + Send + Unpin {

        let data_copy = self.0.read().await;

        let mut save_entries = Vec::with_capacity(data_copy.len());
        for (_, entry) in data_copy.iter() {
            save_entries.push(entry.clone());
        }

        SaveMarketInfos(save_entries)
            .write(buf)
            .await
            .map(drop)
    }
}

#[derive(Debug, Parse)]
pub struct SaveMarketInfos(pub Vec<MarketOrderInfoEntry>);
