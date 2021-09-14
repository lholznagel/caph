use crate::error::ServerError;

use caph_eve_data_wrapper::{AllianceId, CharacterId, CorporationId, EveDataWrapper};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AssetService {
    pool: PgPool
}

impl AssetService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    /// Gets a list of all items for the given [CharacterId]s
    ///
    /// # Params
    ///
    /// `cids` -> List of [CharacterId]s
    ///
    /// # Returns
    ///
    /// All items that are owned by any of the given characters
    ///
    pub async fn assets(
        &self,
        cids: Vec<CharacterId>
    ) -> Result<Vec<Asset>, ServerError> {
        let cids = cids
            .into_iter()
            .map(|x| *x as i32)
            .collect::<Vec<_>>();

        let assets = sqlx::query!("
                SELECT
                    DISTINCT(a.type_id),
                    ARRAY_AGG(DISTINCT a.character_id) AS owners,
                    ARRAY_AGG(DISTINCT a.item_id) AS item_ids,
                    SUM(a.quantity) AS quantity,
                    i.name
                FROM asset a
                LEFT JOIN item i
                    ON i.type_id = a.type_id
                WHERE character_id = ANY($1)
                GROUP BY
                    a.type_id,
                    i.name
                ORDER BY i.name
            ", &cids)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                Asset {
                    type_id: x.type_id.into(),
                    owners: x.owners.unwrap_or_default(),
                    item_ids: x.item_ids.unwrap_or_default(),
                    quantity: x.quantity.unwrap_or_default(),
                    name: x.name.clone()
                }
            })
            .collect::<Vec<_>>();
        Ok(assets)
    }

    /// Gets a list of all blueprints for the given [CharacterId]s
    ///
    /// # Params
    ///
    /// `cids` -> List of [CharacterId]s
    ///
    /// # Returns
    ///
    /// All blueprints that are owned by any of the given characters
    ///
    pub async fn blueprints(
        &self,
        cids: Vec<CharacterId>
    ) -> Result<Vec<Blueprint>, ServerError> {
        let cids = cids
            .into_iter()
            .map(|x| *x as i32)
            .collect::<Vec<_>>();

        let blueprints = sqlx::query!("
                SELECT
                    DISTINCT(a.type_id),
                    ARRAY_AGG(DISTINCT a.character_id) AS owners,
                    ARRAY_AGG(DISTINCT a.item_id) AS item_ids,
                    ab.material_efficiency,
                    ab.time_efficiency,
                    ab.quantity,
                    ab.runs,
                    COUNT(1) AS count,
                    i.name
                FROM asset_blueprint ab
                JOIN asset a
                    ON a.item_id = ab.item_id
                JOIN item i
                    ON a.type_id = i.type_id
                WHERE character_id = ANY($1)
                GROUP BY
                    a.type_id,
                    ab.quantity,
                    ab.material_efficiency,
                    ab.time_efficiency,
                    ab.runs,
                    i.name
                ORDER BY i.name
            ", &cids)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                Blueprint {
                    type_id:             x.type_id,
                    owners:              x.owners.unwrap_or_default(),
                    item_ids:            x.item_ids.unwrap_or_default(),
                    material_efficiency: x.material_efficiency,
                    time_efficiency:     x.time_efficiency,
                    quantity:            x.quantity,
                    count:               x.count.unwrap_or_default(),
                    runs:                x.runs,
                    name:                x.name
                }
            })
            .collect::<Vec<_>>();
        Ok(blueprints)
    }
}

#[derive(Debug, Serialize)]
pub struct Asset {
    pub type_id:       i32,
    pub owners:        Vec<i32>,
    pub item_ids:      Vec<i64>,
    pub quantity:      i64,
    pub name:          String
}

#[derive(Debug, Serialize)]
pub struct Blueprint {
    pub type_id:             i32,
    pub owners:              Vec<i32>,
    pub item_ids:            Vec<i64>,
    pub material_efficiency: i32,
    pub time_efficiency:     i32,
    pub quantity:            i32,
    pub count:               i64,
    pub runs:                i32,
    pub name:                String
}
