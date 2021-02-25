use super::{ItemMaterialCache, ItemMaterialEntry};

use async_trait::async_trait;
use cachem::{CachemError, Parse, Storage};
use std::collections::HashMap;
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite};

#[async_trait]
impl Storage for ItemMaterialCache {
    fn file() -> &'static str {
        "./db/storage/item_material.cachem"
    }

    async fn load<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncBufRead + AsyncRead + Send + Unpin {

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

            *self.0.write().await = map;
        }
        Ok(())
    }

    async fn save<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncWrite + Send + Unpin {

        let data_copy = self.0.read().await;

        let mut save_entries = Vec::with_capacity(data_copy.len());
        for (_, entries) in data_copy.iter() {
            for x in entries {
                save_entries.push(x.clone());
            }
        }

        SaveItemMaterial(save_entries)
            .write(buf)
            .await
            .map(drop)
    }
}

#[derive(Debug, Parse)]
pub struct SaveItemMaterial(pub Vec<ItemMaterialEntry>);
