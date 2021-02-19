use super::RegionCache;

use async_trait::async_trait;
use cachem::{CachemError, Parse, Storage};
use std::collections::HashSet;
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite};
use tokio::sync::RwLock;

#[async_trait]
impl Storage for RegionCache {
    fn file() -> &'static str {
        "./db/storage/regions.cachem"
    }

    async fn load<B>(buf: &mut B) -> Result<Self, CachemError>
        where B: AsyncBufRead + AsyncRead + Send + Unpin {

        if let Ok(entries) = SaveRegion::read(buf).await {
            Ok(RegionCache(RwLock::new(entries.0)))
        } else {
            Ok(RegionCache::default())
        }
    }

    async fn save<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncWrite + Send + Unpin {

        let data_copy = self.0.read().await;
        SaveRegion(data_copy.clone())
            .write(buf)
            .await
            .map(drop)
    }
}

#[derive(Debug, Parse)]
pub struct SaveRegion(pub HashSet<u32>);
