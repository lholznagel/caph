use crate::PlanetSchematicEntry;

use super::{BlueprintCache, BlueprintEntry};

use async_trait::async_trait;
use cachem::{CachemError, Parse, Storage};
use std::collections::HashMap;
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite};

#[async_trait]
impl Storage for BlueprintCache {
    fn file() -> &'static str {
        "./db/storage/blueprints.cachem"
    }

    async fn load<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncBufRead + AsyncRead + Send + Unpin {

        if let Ok(x) = SaveBlueprint::read(buf).await {
            *self.blueprints.write().await = x
                .blueprints
                .into_iter()
                .map(|x| (x.bid, x))
                .collect::<HashMap<_, _>>();
            *self.schematics.write().await = x
                .schematics
                .into_iter()
                .map(|x| (x.psid, x))
                .collect::<HashMap<_, _>>();
        }
        Ok(())
    }

    async fn save<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncWrite + Send + Unpin {

        let blueprints = self
            .blueprints
            .read()
            .await
            .iter()
            .map(|(_, x)| x.clone())
            .collect::<Vec<_>>();
        let schematics = self
            .schematics
            .read()
            .await
            .iter()
            .map(|(_, x)| x.clone())
            .collect::<Vec<_>>();

        SaveBlueprint {
            blueprints,
            schematics
        }
            .write(buf)
            .await?;

        Ok(())
    }
}

#[derive(Debug, Parse)]
pub struct SaveBlueprint {
    pub blueprints: Vec<BlueprintEntry>,
    pub schematics: Vec<PlanetSchematicEntry>,
}
