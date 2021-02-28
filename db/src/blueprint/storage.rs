use super::{BlueprintCache, BlueprintEntry};

use async_trait::async_trait;
use cachem::{CachemError, Parse, Storage};
use std::collections::HashMap;
use std::time::Instant;
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite};

const METRIC_STORAGE_LOAD: &'static str = "storage::blueprint::load";
const METRIC_STORAGE_SAVE: &'static str = "storage::blueprint::save";

#[async_trait]
impl Storage for BlueprintCache {
    fn file() -> &'static str {
        "./db/storage/blueprints.cachem"
    }

    async fn load<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncBufRead + AsyncRead + Send + Unpin {

        let timer = Instant::now();

        if let Ok(entries) = SaveBlueprint::read(buf).await {
            let mut map = HashMap::with_capacity(entries.0.len());
            for entry in entries.0 {
                map.insert(entry.item_id, entry);
            }

            *self.cache.write().await = map;
        }

        self.metrix.send_time(METRIC_STORAGE_LOAD, timer).await;
        Ok(())
    }

    async fn save<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncWrite + Send + Unpin {

        let timer = Instant::now();
        let data_copy = self.cache.read().await;

        let mut save_entries = Vec::with_capacity(data_copy.len());
        for (_, entry) in data_copy.iter() {
            save_entries.push(entry.clone());
        }

        SaveBlueprint(save_entries)
            .write(buf)
            .await?;

        self.metrix.send_time(METRIC_STORAGE_SAVE, timer).await;
        Ok(())
    }
}

#[derive(Debug, Parse)]
pub struct SaveBlueprint(pub Vec<BlueprintEntry>);
