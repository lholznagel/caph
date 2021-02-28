use super::RegionCache;

use async_trait::async_trait;
use cachem::{CachemError, Parse, Storage};
use std::collections::HashSet;
use std::time::Instant;
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite};

const METRIC_STORAGE_LOAD: &'static str = "storage::region::load";
const METRIC_STORAGE_SAVE: &'static str = "storage::region::save";

#[async_trait]
impl Storage for RegionCache {
    fn file() -> &'static str {
        "./db/storage/regions.cachem"
    }

    async fn load<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncBufRead + AsyncRead + Send + Unpin {

        let timer = Instant::now();

        if let Ok(entries) = SaveRegion::read(buf).await {
            *self.cache.write().await = entries.0;
        }

        self.metrix.send_time(METRIC_STORAGE_LOAD, timer).await;
        Ok(())
    }

    async fn save<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncWrite + Send + Unpin {

        let timer = Instant::now();
        let data_copy = self.cache.read().await;
        SaveRegion(data_copy.clone())
            .write(buf)
            .await?;

        self.metrix.send_time(METRIC_STORAGE_SAVE, timer).await;
        Ok(())
    }
}

#[derive(Debug, Parse)]
pub struct SaveRegion(pub HashSet<u32>);
