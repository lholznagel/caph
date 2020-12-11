use crate::{error::CollectorError, metrics::SdeMetrics};

use caph_eve_sde_parser::{
    Blueprint, ParseRequest, ParseResult, Schematic, Station, TypeIds, TypeMaterial, UniqueName,
};
use metrix::Metrics;
use sqlx::{pool::PoolConnection, Executor, Pool, Postgres};
use std::collections::HashMap;
use std::io::Cursor;
use std::time::Instant;

const BATCH_SIZE: usize = 10_000;

pub struct Sde {
    db: Pool<Postgres>,
    metrics: Metrics,
}

impl Sde {
    pub async fn new(db: Pool<Postgres>, metrics: Metrics) -> Self {
        Self {
            db,
            metrics,
        }
    }

    pub async fn background(&mut self) -> Result<(), CollectorError> {
        log::debug!("Fetching sde zip");
        let start = Instant::now();
        let zip = caph_eve_sde_parser::fetch_zip()
            .await
            .map_err(|_| CollectorError::DownloadSdeZip)?;
        self.metrics.duration(SdeMetrics::DOWNLOAD_TIME, start).await;
        log::debug!("Fetched sde zip");

        log::debug!("Parsing sde zip");
        let start = Instant::now();
        let parse_results = caph_eve_sde_parser::from_reader(
            &mut Cursor::new(zip),
            vec![
                ParseRequest::TypeIds,
                ParseRequest::TypeMaterials,
                ParseRequest::UniqueNames,
                ParseRequest::Stations,
                ParseRequest::Blueprints,
                ParseRequest::Schematics,
            ],
        )
        .map_err(CollectorError::SdeParserError)?;
        self.metrics.duration(SdeMetrics::PARSE_TIME, start).await;
        log::debug!("Parsed sde zip");

        let mut conn = self.db.acquire().await?;
        conn.execute("BEGIN").await?;

        self.remove_old("items").await?;
        self.remove_old("item_materials").await?;
        self.remove_old("names").await?;
        self.remove_old("stations").await?;
        self.remove_old("blueprints").await?;
        self.remove_old("blueprint_resources").await?;
        self.remove_old("schematics").await?;

        let start = Instant::now();
        for parse_result in parse_results {
            match parse_result {
                ParseResult::TypeIds(x) => self.items(&mut conn, x).await?,
                ParseResult::TypeMaterials(x) => self.item_materials(&mut conn, x).await?,
                ParseResult::UniqueNames(x) => self.names(&mut conn, x).await?,
                ParseResult::Stations(x) => self.stations(&mut conn, x).await?,
                ParseResult::Blueprints(x) => self.blueprints(&mut conn, x).await?,
                ParseResult::Schematic(x) => self.schematics(&mut conn, x).await?,
            }
        }

        self.metrics
            .duration(SdeMetrics::TOTAL_DB_INSERT_TIME, start)
            .await;
        conn.execute("COMMIT").await?;
        self.metrics
            .current_timestamp(SdeMetrics::LAST_COMPLETE_READOUT)
            .await;
        Ok(())
    }

    async fn items(
        &mut self,
        conn: &mut PoolConnection<Postgres>,
        items: HashMap<u32, TypeIds>,
    ) -> Result<(), CollectorError> {
        log::info!("Starting item import");
        let start = Instant::now();

        let mut skip = 0;
        let items_len = items.len();

        let mut ids = Vec::with_capacity(BATCH_SIZE);
        let mut names = Vec::with_capacity(BATCH_SIZE);
        let mut descriptions = Vec::with_capacity(BATCH_SIZE);
        let mut volumes = Vec::with_capacity(BATCH_SIZE);

        while skip <= items_len {
            for (id, item) in items.iter().skip(skip).take(BATCH_SIZE) {
                let name = item.name.get("en").map(|x| x.clone()).unwrap_or_default();
                let description = if let Some(x) = item.description.clone() {
                    x.get("en").map(|x| x.clone()).unwrap_or("".into())
                } else {
                    "".into()
                };

                ids.push(*id as i32);
                names.push(name);
                descriptions.push(description);
                volumes.push(item.volume.unwrap_or(0f32));
            }

            sqlx::query(
                r#"INSERT INTO items (id, name, description, volume)
                SELECT * FROM UNNEST($1, $2, $3, $4)
                RETURNING id, name, description"#,
            )
            .bind(&ids)
            .bind(&names)
            .bind(&descriptions)
            .bind(&volumes)
            .execute(&mut *conn)
            .await?;

            ids.clear();
            names.clear();
            descriptions.clear();
            volumes.clear();
            skip += BATCH_SIZE;
        }

        self.metrics.duration(SdeMetrics::ITEM_INSERT_TIME, start).await;
        log::info!("Importing items done. Took {}s", start.elapsed().as_secs());
        Ok(())
    }

    async fn item_materials(
        &mut self,
        conn: &mut PoolConnection<Postgres>,
        materials: HashMap<u32, TypeMaterial>,
    ) -> Result<(), CollectorError> {
        log::info!("Starting item material import");
        let start = Instant::now();

        let mut skip = 0;
        let materials_len = materials.len();

        // every reipe has one or more ingridient, so create a larger vec
        let mut ids = Vec::with_capacity(BATCH_SIZE * 3);
        let mut material_ids = Vec::with_capacity(BATCH_SIZE * 3);
        let mut quantities = Vec::with_capacity(BATCH_SIZE * 3);

        while skip <= materials_len {
            for (id, materials) in materials.iter().skip(skip).take(BATCH_SIZE) {
                for material in materials.materials.iter() {
                    ids.push(*id as i32);
                    material_ids.push(material.material_type_id as i32);
                    quantities.push(material.quantity as i64);
                }
            }

            sqlx::query(
                r#"INSERT INTO item_materials (id, material_id, quantity)
                SELECT * FROM UNNEST($1, $2, $3)
                RETURNING id, material_id, quantity"#,
            )
            .bind(&ids)
            .bind(&material_ids)
            .bind(&quantities)
            .execute(&mut *conn)
            .await?;

            ids.clear();
            material_ids.clear();
            quantities.clear();
            skip += BATCH_SIZE;
        }

        self.metrics
            .duration(SdeMetrics::ITEM_MATERIAL_INSERT_TIME, start)
            .await;
        log::info!(
            "Importing item materials done. Took {}s",
            start.elapsed().as_secs()
        );
        Ok(())
    }

    async fn names(
        &mut self,
        conn: &mut PoolConnection<Postgres>,
        names: Vec<UniqueName>,
    ) -> Result<(), CollectorError> {
        log::info!("Starting name import");
        let start = Instant::now();

        let mut skip = 0;
        let names_len = names.len();

        let mut ids = Vec::with_capacity(BATCH_SIZE);
        let mut names_db = Vec::with_capacity(BATCH_SIZE);

        while skip <= names_len {
            for name in names.iter().skip(skip).take(BATCH_SIZE) {
                ids.push(name.item_id as i32);
                names_db.push(name.item_name.clone());
            }

            sqlx::query(
                r#"INSERT INTO names (id, name)
                SELECT * FROM UNNEST($1, $2)
                RETURNING id, name"#,
            )
            .bind(&ids)
            .bind(&names_db)
            .execute(&mut *conn)
            .await?;

            ids.clear();
            names_db.clear();
            skip += BATCH_SIZE;
        }

        self.metrics.duration(SdeMetrics::NAME_INSERT_TIME, start).await;
        log::info!("Importing names done. Took {}s", start.elapsed().as_secs());
        Ok(())
    }

    async fn stations(
        &mut self,
        conn: &mut PoolConnection<Postgres>,
        stations: Vec<Station>,
    ) -> Result<(), CollectorError> {
        log::info!("Starting station import");
        let start = Instant::now();

        let mut skip = 0;
        let stations_len = stations.len();

        let mut station_ids = Vec::with_capacity(BATCH_SIZE);
        let mut constellation_ids = Vec::with_capacity(BATCH_SIZE);
        let mut region_ids = Vec::with_capacity(BATCH_SIZE);
        let mut solar_system_ids = Vec::with_capacity(BATCH_SIZE);
        let mut security = Vec::with_capacity(BATCH_SIZE);

        while skip <= stations_len {
            for station in stations.iter().skip(skip).take(BATCH_SIZE) {
                station_ids.push(station.station_id);
                constellation_ids.push(station.constellation_id);
                region_ids.push(station.region_id);
                solar_system_ids.push(station.solar_system_id);
                security.push(station.security);
            }

            sqlx::query(
                r#"INSERT INTO stations (station_id, constellation_id, region_id, system_id, security)
                SELECT * FROM UNNEST($1, $2, $3, $4, $5)
                RETURNING station_id, constellation_id, region_id, system_id, security"#,
            )
            .bind(&station_ids)
            .bind(&constellation_ids)
            .bind(&region_ids)
            .bind(&solar_system_ids)
            .bind(&security)
            .execute(&mut *conn)
            .await?;

            station_ids.clear();
            constellation_ids.clear();
            region_ids.clear();
            solar_system_ids.clear();
            security.clear();
            skip += BATCH_SIZE;
        }

        self.metrics
            .duration(SdeMetrics::STATION_INSERT_TIME, start)
            .await;
        log::info!(
            "Importing stations done. Took {}s",
            start.elapsed().as_secs()
        );
        Ok(())
    }

    async fn blueprints(
        &mut self,
        conn: &mut PoolConnection<Postgres>,
        blueprints: HashMap<u32, Blueprint>,
    ) -> Result<(), CollectorError> {
        let start = Instant::now();
        let blueprints = blueprints
            .into_iter()
            .map(|(_, b)| b)
            .collect::<Vec<Blueprint>>();

        for blueprint in blueprints {
            if let Some(x) = blueprint.activities.manufacturing {
                sqlx::query("INSERT INTO blueprints (blueprint_id, time) VALUES ($1, $2)")
                    .bind(blueprint.blueprint_type_id)
                    .bind(x.time as i32)
                    .execute(&mut *conn)
                    .await?;

                for material in x.materials.unwrap_or_default() {
                    sqlx::query(r#"INSERT INTO blueprint_resources (blueprint_id, material_id, quantity, is_product) VALUES ($1, $2, $3, false)"#)
                        .bind(blueprint.blueprint_type_id)
                        .bind(material.type_id)
                        .bind(material.quantity)
                        .execute(&mut *conn)
                        .await?;
                }

                for product in x.products.unwrap_or_default() {
                    sqlx::query(r#"INSERT INTO blueprint_resources (blueprint_id, material_id, quantity, is_product) VALUES ($1, $2, $3, true)"#)
                        .bind(blueprint.blueprint_type_id)
                        .bind(product.type_id)
                        .bind(product.quantity)
                        .execute(&mut *conn)
                        .await?;
                }
            }
        }

        self.metrics
            .duration(SdeMetrics::BLUEPRINT_INSERT_TIME, start)
            .await;
        log::info!(
            "Importing blueprints done. Took {}s",
            start.elapsed().as_secs()
        );

        Ok(())
    }

    async fn schematics(
        &mut self,
        conn: &mut PoolConnection<Postgres>,
        schematics: HashMap<u32, Schematic>,
    ) -> Result<(), CollectorError> {
        let start = Instant::now();

        for (id, x) in schematics {
            sqlx::query("INSERT INTO schematics (schematic_id, time) VALUES ($1, $2)")
                .bind(id)
                .bind(x.cycle_time)
                .execute(&mut *conn)
                .await?;

            for (type_id, y) in x.types {
                sqlx::query(
                    r#"
                INSERT INTO schematic_resources (schematic_id, material_id, quantity, is_input)
                VALUES ($1, $2, $3, $4)
                "#,
                )
                .bind(id)
                .bind(type_id)
                .bind(y.quantity)
                .bind(y.is_input)
                .execute(&mut *conn)
                .await?;
            }
        }

        self.metrics
            .duration(SdeMetrics::SCHEMATIC_INSERT_TIME, start)
            .await;
        log::info!(
            "Importing schematics done. Took {}s",
            start.elapsed().as_secs()
        );

        Ok(())
    }

    async fn remove_old(&mut self, table: &str) -> Result<(), CollectorError> {
        let start = Instant::now();
        log::debug!("Removing all unlocked items in {}", table);

        let mut conn = self.db.acquire().await?;
        sqlx::query(&format!("DELETE FROM {}", table))
            .execute(&mut conn)
            .await?;

        log::debug!("Removed all unlocked items");
        self.metrics.duration(SdeMetrics::CLEANUP_TIME, start).await;
        Ok(())
    }
}
