use crate::ServerError;

use caph_connector::{CategoryId, CharacterId, GroupId, ItemId, LocationId, TypeId};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

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
    ) -> Result<Option<Asset>, ServerError> {
        // https://github.com/launchbadge/sqlx/issues/367
        let entry = sqlx::query("
                SELECT
                    a.*,
                    ab.material_efficiency,
                    ab.time_efficiency,
                    ab.runs = -1 AS original,
                    ab.runs,
                    i.name,
                    i.volume,
                    i.category_id,
                    i.group_id
                FROM asset a
                LEFT JOIN item i
                    ON a.type_id = i.type_id
                LEFT JOIN asset_blueprint ab
                    ON ab.item_id = a.item_id
                WHERE a.character_id = ANY(
                    SELECT character_id
                    FROM character
                    WHERE character_id = $1 OR character_main = $1
                )
                  AND a.item_id = $2
            ")
            .bind(*cid)
            .bind(*iid)
            .map(|x: PgRow| {
                let type_id: i32      = x.get("type_id");
                let category_id: i32  = x.get("category_id");
                let group_id: i32     = x.get("group_id");
                let character_id: i32 = x.get("character_id");

                let item_id: i64      = x.get("item_id");
                let location_id: i64  = x.get("location_id");

                let reference_id: Option<i64> = x.get("reference_id");

                Asset {
                    type_id:       type_id.into(),
                    item_id:       item_id.into(),
                    location_id:   location_id.into(),
                    reference_id:  reference_id.map(|x| x.into()),
                    quantity:      x.get("quantity"),
                    owner:         character_id.into(),
                    volume:        x.get("volume"),
                    category_id:   category_id.into(),
                    group_id:      group_id.into(),
                    location_flag: x.get("location_flag"),
                    name:          x.get("name"),
                    material_eff:  x.get("material_efficiency"),
                    time_eff:      x.get("time_efficiency"),
                    original:      x.get("original"),
                    runs:          x.get("runs")
                }
            })
            .fetch_optional(&self.pool)
            .await?;
        Ok(entry)
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

        // https://github.com/launchbadge/sqlx/issues/367
        let assets = sqlx::query("
                SELECT
                    DISTINCT(a.type_id),
                    ARRAY_AGG(DISTINCT a.character_id) AS owners,
                    ARRAY_AGG(DISTINCT a.item_id) AS item_ids,
                    ARRAY_AGG(DISTINCT a.location_id) AS location_ids,
                    SUM(a.quantity) AS quantity,
                    ab.material_efficiency,
                    ab.time_efficiency,
                    ab.runs = -1 AS original,
                    ab.runs,
                    i.name,
                    i.volume,
                    i.category_id,
                    i.group_id
                FROM asset a
                LEFT JOIN asset_blueprint ab
                    ON ab.item_id = a.item_id
                LEFT JOIN asset_name an
                    ON an.item_id = a.item_id
                LEFT JOIN item i
                    ON i.type_id = a.type_id
                WHERE a.character_id = ANY($1)
                  AND (
                       $2::BIGINT IS NULL
                    OR a.item_id = $2::BIGINT
                  )
                  AND (
                       $3::VARCHAR IS NULL
                    OR i.name ILIKE '%' || $3::VARCHAR || '%'
                  )
                  AND (
                       $4::VARCHAR IS NULL
                    OR an.name ILIKE '%' || $4::VARCHAR || '%'
                  )
                  AND (
                       $5::INTEGER IS NULL
                    OR i.category_id = $5::INTEGER
                  )
                  AND (
                       $6::INTEGER IS NULL
                    OR i.group_id = $6::INTEGER
                  )
                GROUP BY
                    a.type_id,
                    ab.quantity,
                    ab.material_efficiency,
                    ab.time_efficiency,
                    ab.runs,
                    i.name,
                    i.volume,
                    i.category_id,
                    i.group_id
                ORDER BY i.name
            ")
            .bind(&cids)
            .bind(filter.item_id)
            .bind(filter.name)
            .bind(filter.asset_name)
            .bind(filter.category)
            .bind(filter.group)
            .map(|x: PgRow| {
                let type_id: i32     = x.get("type_id");
                let category_id: i32 = x.get("category_id");
                let group_id: i32    = x.get("group_id");

                CharacterAsset {
                    type_id:      type_id.into(),
                    owners:       x.get("owners"),
                    item_ids:     x.get("item_ids"),
                    location_ids: x.get("location_ids"),
                    quantity:     x.get("quantity"),
                    volume:       x.get("volume"),
                    name:         x.get("name"),
                    category_id:  category_id.into(),
                    group_id:     group_id.into(),

                    material_eff: x.get("material_efficiency"),
                    time_eff:     x.get("time_efficiency"),
                    original:     x.get("original"),
                    runs:         x.get("runs")
                }
            })
            .fetch_all(&self.pool)
            .await?;
        Ok(assets)
    }

    pub async fn blueprint_material(
        &self,
        tid: TypeId
    ) -> Result<Vec<BlueprintMaterial>, ServerError> {
        let entries = sqlx::query!(r#"
                SELECT
                    bm.type_id    AS "type_id!",
                    bm.quantity   AS "quantity!",
                    bm.is_product AS "is_product!"
                FROM blueprint b
                JOIN blueprint_material bm
                  ON b.id = bm.blueprint
                WHERE b.type_id = $1
                  AND (bm.activity = 2 OR bm.activity = 3)
            "#,
                *tid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                BlueprintMaterial {
                    type_id:    x.type_id.into(),
                    quantity:   x.quantity,
                    is_product: x.is_product
                }
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    pub async fn blueprint_material_bulk(
        &self,
        tids:       Vec<TypeId>,
        is_product: Option<bool>
    ) -> Result<HashMap<TypeId, BlueprintMaterial>, ServerError> {
        let tids = tids.into_iter().map(|x| *x).collect::<Vec<_>>();

        let entries = sqlx::query!(r#"
                SELECT
                    bm.type_id    AS "type_id!",
                    bm.quantity   AS "quantity!",
                    bm.is_product AS "is_product!"
                FROM blueprint b
                JOIN blueprint_material bm
                  ON b.id = bm.blueprint
                WHERE b.type_id = ANY($1)
                  AND (bm.activity = 2 OR bm.activity = 3)
                  AND (
                    $2::BOOLEAN IS NULL OR bm.is_product = $2::BOOLEAN
                  )
            "#,
                &tids,
                is_product
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                (x.type_id.into(), BlueprintMaterial {
                    type_id:    x.type_id.into(),
                    quantity:   x.quantity,
                    is_product: x.is_product
                })
            })
            .collect::<HashMap<_, _>>();
        Ok(entries)
    }

    pub async fn blueprint_tree(
        &self,
        tid: TypeId
    ) -> Result<serde_json::Value, ServerError> {
        let entry = sqlx::query!("
                SELECT tree
                FROM blueprint_tree
                WHERE type_id = $1
            ",
                *tid
            )
            .fetch_optional(&self.pool)
            .await?;

        if let Some(e) = entry {
            Ok(e.tree)
        } else {
            Err(ServerError::NotFound)
        }
    }

    pub async fn blueprint_flat(
        &self,
        tid: TypeId
    ) -> Result<Vec<BlueprintFlat>, ServerError> {
        let entries = sqlx::query!("
                SELECT bf.type_id, bf.mtype_id, i.name
                FROM blueprint_flat bf
                JOIN item i
                  ON i.type_id = mtype_id
                WHERE bf.type_id = $1
            ",
                *tid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                BlueprintFlat {
                    type_id:  x.type_id.into(),
                    mtype_id: x.mtype_id.into(),
                    name:     x.name,
                }
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    pub async fn general_assets_buildable(
        &self,
    ) -> Result<Vec<AssetBuildable>, ServerError> {
        let mut blueprints = sqlx::query!("
                SELECT i.type_id, i.name
                FROM blueprint_material bm
                JOIN item i
                  ON i.type_id = bm.type_id
                WHERE bm.is_product = TRUE
                  AND (bm.activity = 2 OR bm.activity = 3)
                ORDER BY i.name
            ")
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                AssetBuildable {
                    type_id: x.type_id.into(),
                    name:    x.name
                }
            })
            .collect::<Vec<_>>();
        let schematics = sqlx::query!("
                SELECT DISTINCT(i.type_id), i.name
                FROM schematic_material sm
                JOIN item i
                  ON i.type_id = sm.type_id
                WHERE sm.is_input = FALSE
                ORDER BY i.name
            ")
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                AssetBuildable {
                    type_id: x.type_id.into(),
                    name:    x.name
                }
            })
            .collect::<Vec<_>>();

        blueprints.extend(schematics);
        Ok(blueprints)
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

    pub async fn resolve_id_from_name_bulk(
        &self,
        names:  Vec<String>,
        filter: ResolveIdNameFilter
    ) -> Result<Vec<ResolveIdName>, ServerError> {
        let entries = sqlx::query!("
                SELECT name, type_id
                FROM item
                WHERE name = ANY($1)
                  AND (
                       $2::BOOLEAN IS NULL
                    OR (
                        SELECT type_id
                        FROM blueprint_material bm
                        WHERE bm.type_id = item.type_id
                          AND is_product = TRUE
                    ) IS NOT NULL
                  )
            ",
                &names,
                filter.has_blueprint
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| ResolveIdName {
                name:    x.name,
                type_id: x.type_id.into()
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Asset {
    pub type_id:       TypeId,
    pub item_id:       ItemId,
    pub location_id:   LocationId,
    pub owner:         CharacterId,
    pub quantity:      i32,
    pub volume:        f32,
    pub category_id:   CategoryId,
    pub group_id:      GroupId,

    pub location_flag: String,
    pub name:          String,

    pub reference_id:  Option<ItemId>,
    pub material_eff:  Option<i32>,
    pub time_eff:      Option<i32>,
    pub original:      Option<bool>,
    pub runs:          Option<i32>,
}

/// Assets including all characters -> multiple owners
#[derive(Debug, Serialize)]
pub struct CharacterAsset {
    pub type_id:       TypeId,
    pub owners:        Vec<i32>,
    pub item_ids:      Vec<i64>,
    pub location_ids:  Vec<i64>,
    pub quantity:      i64,
    pub volume:        f32,
    pub category_id:   CategoryId,
    pub group_id:      GroupId,
    pub name:          String,

    pub material_eff:  Option<i32>,
    pub time_eff:      Option<i32>,
    pub original:      Option<bool>,
    pub runs:          Option<i32>,
}

#[derive(Debug, Default, Deserialize)]
pub struct AssetFilter {
    pub item_id:    Option<i64>,
    pub name:       Option<String>,
    pub asset_name: Option<String>,
    pub category:   Option<i32>,
    pub group:      Option<i32>,
    pub owner:      Option<CharacterId>,
}

#[derive(Debug, Serialize)]
pub struct AssetBuildable {
    pub type_id: TypeId,
    pub name:    String
}

#[derive(Clone, Debug, Serialize)]
pub struct ResolveIdName {
    pub name:    String,
    pub type_id: TypeId,
}

#[derive(Debug, Deserialize)]
pub struct ResolveIdNameFilter {
    pub has_blueprint: Option<bool>
}

#[derive(Clone, Debug, Serialize)]
pub struct BlueprintRaw {
    pub type_id:  TypeId,
    pub group_id: GroupId,
    pub quantity: i32,
    pub name:     String,
}

#[derive(Debug, Serialize)]
pub struct BlueprintFlat {
    pub type_id:  TypeId,
    pub mtype_id: GroupId,
    pub name:     String,
}

#[derive(Debug, Serialize)]
pub struct BlueprintMaterial {
    pub type_id:    TypeId,
    pub quantity:   i32,
    pub is_product: bool
}
