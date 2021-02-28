use super::{ItemMaterialCache, ItemMaterialEntry};

use async_trait::async_trait;
use cachem::{CachemError, Parse, Storage};
use std::collections::HashMap;
use std::time::Instant;
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite};

const METRIC_STORAGE_LOAD: &'static str = "storage::item_material::load";
const METRIC_STORAGE_SAVE: &'static str = "storage::item_material::save";

#[async_trait]
impl Storage for ItemMaterialCache {
    fn file() -> &'static str {
        "./db/storage/item_material.cachem"
    }

    async fn load<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncBufRead + AsyncRead + Send + Unpin {

        let timer = Instant::now();

        if let Ok(entries) = SaveItemMaterial::read(buf).await {
            let mut map = HashMap::with_capacity(entries.0.len());
            for x in entries.0.iter() {
                map
                    .entry(x.item_id)
                    .and_modify(|entry: &mut Vec<ItemMaterialEntry>| {
                        if !entry.contains(&x) {
                            entry.push(x.clone());
                        }
                    })
                    .or_insert({
                        vec![*x]
                    });
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
        for (_, entries) in data_copy.iter() {
            for x in entries {
                save_entries.push(x.clone());
            }
        }

        SaveItemMaterial(save_entries)
            .write(buf)
            .await?;

        self.metrix.send_time(METRIC_STORAGE_SAVE, timer).await;
        Ok(())
    }
}

#[derive(Debug, Parse)]
pub struct SaveItemMaterial(pub Vec<ItemMaterialEntry>);
