use crate::ServerError;

use caph_connector::{CharacterId, ItemId, LocationId, TypeId};
use serde::{Deserialize, Serialize};
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

    pub async fn asset(
        &self,
        cid: CharacterId,
        iid: ItemId
    ) -> Result<Asset, ServerError> {
        let entry = sqlx::query!("
                SELECT a.*, i.name
                FROM asset a
                JOIN item i
                ON a.type_id = i.type_id
                WHERE character_id = ANY(
                    SELECT DISTINCT character_id
                    FROM login
                    WHERE character_id = $1 OR character_main = $1
                    AND character_id IS NOT NULL
                )
                  AND item_id = $2
            ",
                *cid,
                *iid
            )
            .fetch_optional(&self.pool)
            .await?;

        if let Some(e) = entry {
            let asset = Asset {
                type_id:       e.type_id.into(),
                item_id:       e.item_id.into(),
                location_id:   e.location_id.into(),
                reference_id:  e.reference_id.map(|x| x.into()),
                quantity:      e.quantity,
                owner:         e.character_id.into(),
                location_flag: e.location_flag,
                name:          e.name
            };
            Ok(asset)
        } else {
            Err(ServerError::NotFound)
        }
    }

    pub async fn asset_name(
        &self,
        cid: CharacterId,
        iid: ItemId
    ) -> Result<String, ServerError> {
        let entry = sqlx::query!("
                SELECT name
                FROM asset_name
                WHERE character_id = ANY(
                    SELECT DISTINCT character_id
                    FROM login
                    WHERE character_id = $1 OR character_main = $1
                    AND character_id IS NOT NULL
                )
                  AND item_id = $2
            ",
                *cid,
                *iid
            )
            .fetch_optional(&self.pool)
            .await?;

        if let Some(e) = entry {
            Ok(e.name)
        } else {
            Err(ServerError::NotFound)
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
        cids:   Vec<CharacterId>,
        filter: AssetFilter
    ) -> Result<Vec<CharacterAsset>, ServerError> {
        let cids = if let Some(x) = filter.owner {
            vec![*x]
        } else {
            cids.into_iter().map(|x| *x).collect::<Vec<_>>()
        };

        let assets = sqlx::query!("
                SELECT
                    DISTINCT(a.type_id),
                    ARRAY_AGG(DISTINCT a.character_id) AS owners,
                    ARRAY_AGG(DISTINCT a.item_id) AS item_ids,
                    ARRAY_AGG(DISTINCT a.location_id) AS location_ids,
                    SUM(a.quantity) AS quantity,
                    i.name,
                    i.category_id,
                    i.group_id
                FROM asset a
                LEFT JOIN item i
                    ON i.type_id = a.type_id
                LEFT JOIN asset_name an
                    ON an.item_id = a.item_id
                WHERE a.character_id = ANY($1)
                  AND (
                       $2::VARCHAR IS NULL
                    OR i.name ILIKE '%' || $2::VARCHAR || '%'
                  )
                  AND (
                       $3::VARCHAR IS NULL
                    OR an.name ILIKE '%' || $3::VARCHAR || '%'
                  )
                  AND (
                       $4::INTEGER IS NULL
                    OR i.category_id = $4::INTEGER
                  )
                GROUP BY
                    a.type_id,
                    i.name,
                    i.category_id,
                    i.group_id
                ORDER BY i.name
            ",
                &cids,
                filter.name,
                filter.asset_name,
                filter.category,
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                CharacterAsset {
                    type_id: x.type_id.into(),
                    owners: x.owners.unwrap_or_default(),
                    item_ids: x.item_ids.unwrap_or_default(),
                    location_ids: x.location_ids.unwrap_or_default(),
                    quantity: x.quantity.unwrap_or_default(),
                    name: x.name.clone(),
                    category_id: x.category_id.into(),
                    group_id: x.group_id.into()
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
        cids:   Vec<CharacterId>,
        filter: BlueprintFilter
    ) -> Result<Vec<AccountBlueprint>, ServerError> {
        let cids = if let Some(x) = filter.owner {
            vec![*x]
        } else {
            cids.into_iter().map(|x| *x).collect::<Vec<_>>()
        };

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
                  AND (
                       $2::VARCHAR IS NULL
                    OR i.name LIKE '%' || $2::VARCHAR || '%'
                  )
                  AND (
                       $3::INTEGER IS NULL
                    OR ab.material_efficiency = $3
                  )
                  AND (
                       $4::INTEGER IS NULL
                    OR ab.time_efficiency = $4
                  )
                GROUP BY
                    a.type_id,
                    ab.quantity,
                    ab.material_efficiency,
                    ab.time_efficiency,
                    ab.runs,
                    i.name
                ORDER BY i.name
            ",
                &cids,
                filter.name,
                filter.material_eff,
                filter.time_eff
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                AccountBlueprint {
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

    pub async fn blueprint_material(
        &self,
        tid: TypeId,
    ) -> Result<Vec<BlueprintMaterial>, ServerError> {
        let bps = sqlx::query!("
                SELECT
                    i.name,
                    bm.*
                FROM blueprint b
                JOIN blueprint_material bm ON b.id = bm.blueprint
                JOIN item i ON bm.type_id = i.type_id
                WHERE b.type_id   = $1
                  AND bm.activity = 2
            ",
                *tid,
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                BlueprintMaterial {
                    type_id:    x.type_id,
                    activity:   x.activity,
                    quantity:   x.quantity,
                    is_product: x.is_product,
                    name:       x.name,
                }
            })
            .collect::<Vec<_>>();
        Ok(bps)
    }

    pub async fn blueprint_product(
        &self,
        tid: TypeId,
    ) -> Result<Vec<BlueprintMaterial>, ServerError> {
        let bps = sqlx::query!("
                SELECT
                    i.name,
                    bm.*
                FROM blueprint b
                JOIN blueprint_material bm ON b.id = bm.blueprint
                JOIN item i ON bm.type_id = i.type_id
                WHERE b.type_id     = $1
                  AND bm.activity   = 2
                  AND bm.is_product = TRUE
            ",
                *tid,
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                BlueprintMaterial {
                    type_id:    x.type_id,
                    activity:   x.activity,
                    quantity:   x.quantity,
                    is_product: x.is_product,
                    name:       x.name,
                }
            })
            .collect::<Vec<_>>();
        Ok(bps)
    }

    pub async fn character_blueprint(
        &self,
        cids: Vec<CharacterId>,
        tid:  TypeId,
        iid:  ItemId,
    ) -> Result<CharacterBlueprint, ServerError> {
        let cids = cids.into_iter().map(|x| *x).collect::<Vec<_>>();

        let bp = sqlx::query!("
                SELECT
                    a.type_id,
                    a.item_id,
                    ab.material_efficiency,
                    ab.time_efficiency,
                    ab.quantity,
                    ab.runs,
                    i.name
                FROM asset_blueprint ab
                JOIN asset a
                    ON a.item_id = ab.item_id
                JOIN item i
                    ON a.type_id = i.type_id
                WHERE character_id = ANY($1)
                  AND a.type_id = $2
                  AND a.item_id = $3
                ORDER BY i.name
            ",
                &cids,
                *tid,
                *iid
            )
            .fetch_optional(&self.pool)
            .await?;
        if let Some(bp) = bp {
            let bp = CharacterBlueprint {
                type_id:             bp.type_id,
                item_id:             bp.item_id,
                material_efficiency: bp.material_efficiency,
                time_efficiency:     bp.time_efficiency,
                quantity:            bp.quantity,
                runs:                bp.runs,
                name:                bp.name
            };
            Ok(bp)
        } else {
            Err(ServerError::NotFound)
        }
    }

    pub async fn item_location(
        &self,
        cids: Vec<CharacterId>,
        iid:  ItemId
    ) -> Result<Option<LocationId>, ServerError> {
        let cids = cids.into_iter().map(|x| *x).collect::<Vec<_>>();

        let entry = sqlx::query!("
                SELECT location_id
                FROM asset a
                WHERE a.character_id = ANY($1)
                  AND a.item_id = $2
            ",
                &cids,
                *iid
            )
            .fetch_optional(&self.pool)
            .await?;
        if let Some(e) = entry {
            Ok(Some(e.location_id.into()))
        } else {
            Ok(None)
        }
    }

    pub async fn station_name(
        &self,
        sid: LocationId
    ) -> Result<Option<String>, ServerError> {
        let entry = sqlx::query!("
                SELECT name
                FROM station
                WHERE id = $1
            ",
                *sid
            )
            .fetch_optional(&self.pool)
            .await?;
        if let Some(e) = entry {
            Ok(Some(e.name))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Asset {
    pub type_id:       TypeId,
    pub item_id:       ItemId,
    pub location_id:   LocationId,
    pub owner:         CharacterId,
    pub quantity:      i32,
    pub location_flag: String,
    pub name:          String,

    pub reference_id:  Option<ItemId>,
}

/// Assets including all characters -> multiple owners
#[derive(Debug, Serialize)]
pub struct CharacterAsset {
    pub type_id:       i32,
    pub owners:        Vec<i32>,
    pub item_ids:      Vec<i64>,
    pub location_ids:  Vec<i64>,
    pub quantity:      i64,
    pub name:          String,

    pub category_id: caph_connector::CategoryId,
    pub group_id:    caph_connector::GroupId,
}

#[derive(Debug, Serialize)]
pub struct AccountBlueprint {
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

#[derive(Debug, Serialize)]
pub struct BlueprintMaterial {
    pub type_id:             i32,
    pub activity:            i16,
    pub quantity:            i32,
    pub is_product:          bool,
    pub name:                String
}

#[derive(Debug, Serialize)]
pub struct CharacterBlueprint {
    pub type_id:             i32,
    pub item_id:             i64,
    pub material_efficiency: i32,
    pub time_efficiency:     i32,
    pub quantity:            i32,
    pub runs:                i32,
    pub name:                String
}

#[derive(Debug, Default, Deserialize)]
pub struct AssetFilter {
    pub name:       Option<String>,
    pub asset_name: Option<String>,
    pub category:   Option<i32>,
    pub owner:      Option<CharacterId>,
}

#[derive(Debug, Default, Deserialize)]
pub struct BlueprintFilter {
    pub name:         Option<String>,
    pub owner:        Option<CharacterId>,
    pub material_eff: Option<i32>,
    pub time_eff:     Option<i32>,
}
