use crate::error::*;
use crate::eve::*;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Serialize)]
pub struct GroupId(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupData {
    pub category_id: u32,
    pub group_id: GroupId,
    pub name: String,
    pub published: bool,
    pub types: Vec<TypeId>,
}

impl Eve {
    pub async fn fetch_groups(&self, group_ids: Vec<GroupId>) -> Result<Vec<GroupData>> {
        let mut result = Vec::with_capacity(group_ids.len());

        for type_id in group_ids {
            let response = self
                .fetch(&format!(
                    "universe/groups/{}?datasource=tranquility",
                    type_id.0
                ))
                .await?;

            if response.status() == 404 {
                log::warn!("GroupId {} does not exist. Skipping.", type_id.0);
                continue;
            }

            log::debug!("Downloaded GroupId {}", type_id.0);
            let type_data = response.json().await?;
            result.push(type_data);
        }

        log::debug!("Downloaded all given GroupIds");
        Ok(result)
    }

    pub async fn fetch_group_ids(&self) -> Result<Vec<TypeId>> {
        self.fetch_ids("universe/groups/?datasource=tranquility&page=")
            .await
    }
}
