use std::collections::HashMap;

use caph_connector::TypeId;
use sqlx::PgPool;

use crate::Error;

use super::Dependency;

/// Maps product [TypeId] to a dependency containing all its required materials
#[derive(Clone, Debug)]
pub struct DependencyCache(HashMap<TypeId, Dependency>);

impl DependencyCache {
    /// Creates a new cache instance.
    /// 
    /// # Params
    /// 
    /// * `pool` -> Postgres connection
    /// 
    /// # Errors
    /// 
    /// If populating the cache fails.
    /// 
    /// Returns
    /// 
    /// Populated cache
    /// 
    pub async fn new(pg: PgPool) -> Result<Self, Error> {
        let entries = Self::populate(pg).await?;
        Ok(Self(entries))
    }

    /// Gets all required materials for a product.
    /// 
    /// # Params
    /// 
    /// * `key` -> [TypeId] of the product, for example 4312 for Oxygen Fuel Block
    /// 
    /// # Returns
    /// 
    /// - [Option::Some] if the [TypeId] matches a product, containing all
    ///                  required materials
    /// - [Option::None] if there is no such product
    ///
    pub fn get(&self, key: &TypeId) -> Option<&Dependency> {
        self.0.get(key)
    }

    /// Populates the cache with all required information.
    /// 
    /// # Params
    /// 
    /// * `pool` -> Postgres connection
    /// 
    /// # Errors
    /// 
    /// If the database access fails
    /// 
    /// # Returns
    /// 
    /// List of product [TypeId] to materials
    /// 
    async fn populate(
        pg: PgPool
    ) -> Result<HashMap<TypeId, Dependency>, Error> {
        let mut entries = HashMap::new();
        // TODO: different query needed
        // Compnents have the wrong time assigned to them 
        // Example: Fullerides -> has he right time, Nitrogen Fuel blocks have the wrong -> Nitrogen materials have the right time (0)
        
        sqlx::query!(r#"
                SELECT
                    bman.ptype_id  AS "ptype_id!",
                    bman.quantity  AS "product_quantity!",
                    bman.time      AS "product_time!",
                    i.name         AS "product_name!",
                    i.category_id  AS "product_category_id!",
                    i.group_id     AS "product_group_id!",
                    bm.mtype_id    AS "mtype_id!",
                    bm.quantity    AS "material_quantity!",
                    ii.name        AS "material_name!",
                    ii.category_id AS "material_category_id!",
                    ii.group_id    AS "material_group_id!"
                FROM blueprint_manufacture bman
                JOIN blueprint_materials bm
                  ON bm.bp_id = bman.bp_id
                JOIN items i
                  ON bman.ptype_id = i.type_id
                JOIN items ii
                  ON bm.mtype_id = ii.type_id
            "#)
            .fetch_all(&pg)
            .await?
            .into_iter()
            .for_each(|x| {
                let material = Dependency {
                    name:             x.material_name,
                    ptype_id:         x.mtype_id.into(),
                    category_id:      x.material_category_id.into(),
                    group_id:         x.material_group_id.into(),
                    products:         x.material_quantity,
                    products_base:    x.material_quantity,
                    products_per_run: 0,
                    time:             0,
                    time_per_run:     0,
                    components:       Vec::new()
                };

                entries
                    .entry(x.ptype_id.into())
                    .and_modify(|e: &mut Dependency|
                        e.components.push(material.clone())
                    )
                    .or_insert(Dependency {
                        name:             x.product_name,
                        ptype_id:         x.ptype_id.into(),
                        category_id:      x.product_category_id.into(),
                        group_id:         x.product_group_id.into(),
                        products:         0,
                        products_base:    0,
                        products_per_run: x.product_quantity,
                        time:             x.product_time as i64,
                        time_per_run:     x.product_time as i64,
                        components:       vec![material]
                    });
            });

        Ok(entries)
    }
}

#[cfg(test)]
mod dependency_cache_tests {
    use sqlx::postgres::PgPoolOptions;

    use super::*;

    async fn instance() -> DependencyCache {
        dotenv::dotenv().ok();

        let pg_addr = std::env::var("DATABASE_URL")
            .unwrap();
        let pool = PgPoolOptions::new()
            .connect(&pg_addr)
            .await
            .unwrap();

        DependencyCache::new(pool).await.unwrap()
    }

    #[tokio::test]
    async fn dependency_oxygen_fuel_blocks() {
        let instance = instance().await;
        let mut dep = instance.get(&4312.into()).unwrap().clone();
        dep.components.sort_by_key(|x| x.ptype_id);

        assert_eq!(dep.components.len(), 9);

        assert_eq!(dep.components[0].ptype_id, 44.into());
        assert_eq!(dep.components[0].products, 4);
        assert_eq!(dep.components[0].components.len(), 0);

        assert_eq!(dep.components[1].ptype_id, 3683.into());
        assert_eq!(dep.components[1].products, 22);
        assert_eq!(dep.components[1].components.len(), 0);

        assert_eq!(dep.components[2].ptype_id, 3689.into());
        assert_eq!(dep.components[2].products, 4);
        assert_eq!(dep.components[2].components.len(), 0);

        assert_eq!(dep.components[3].ptype_id, 9832.into());
        assert_eq!(dep.components[3].products, 9);
        assert_eq!(dep.components[3].components.len(), 0);

        assert_eq!(dep.components[4].ptype_id, 9848.into());
        assert_eq!(dep.components[4].products, 1);
        assert_eq!(dep.components[4].components.len(), 0);

        assert_eq!(dep.components[5].ptype_id, 16272.into());
        assert_eq!(dep.components[5].products, 170);
        assert_eq!(dep.components[5].components.len(), 0);

        assert_eq!(dep.components[6].ptype_id, 16273.into());
        assert_eq!(dep.components[6].products, 350);
        assert_eq!(dep.components[6].components.len(), 0);

        assert_eq!(dep.components[7].ptype_id, 16275.into());
        assert_eq!(dep.components[7].products, 20);
        assert_eq!(dep.components[7].components.len(), 0);

        assert_eq!(dep.components[8].ptype_id, 17887.into());
        assert_eq!(dep.components[8].products, 450);
        assert_eq!(dep.components[8].components.len(), 0);
    }
}
