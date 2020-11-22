use crate::metrics::SdeMetrics;

use async_std::fs;
use async_std::fs::File;
use async_std::io::prelude::*;
use caph_eve_sde_parser::{ParseRequest, ParseResult, Station, TypeIds, TypeMaterial, UniqueName};
use sqlx::{pool::PoolConnection, Executor, Pool, Postgres};
use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;
use std::time::Instant;

const BATCH_SIZE: usize = 1000;

pub struct Sde {
    db: Pool<Postgres>,
    metrics: SdeMetrics,
}

impl Sde {
    pub fn new(db: Pool<Postgres>, metrics: SdeMetrics) -> Self {
        Self { db, metrics }
    }

    pub async fn background(&mut self) -> Result<(), ()> {
        // when ther is no new checksum return early
        if !self.has_new_checksum().await.unwrap() {
            self.metrics.current_time(SdeMetrics::LAST_CHECKSUM_CHECK);
            log::debug!("No new sde.zip available.");
            return Ok(());
        }
        self.metrics.current_time(SdeMetrics::LAST_CHECKSUM_CHECK);

        log::debug!("Fetching sde zip");
        let start = Instant::now();
        let zip = caph_eve_sde_parser::fetch_zip().await.unwrap();
        self.metrics.set_timing(SdeMetrics::DOWNLOAD_TIME, start);
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
            ],
        )
        .unwrap();
        self.metrics.set_timing(SdeMetrics::PARSE_TIME, start);
        log::debug!("Parsed sde zip");

        let mut conn = self.db.acquire().await.unwrap();
        conn.execute("BEGIN").await.unwrap();

        self.remove_old("items".into()).await.unwrap();
        self.remove_old("item_materials".into()).await.unwrap();
        self.remove_old("names".into()).await.unwrap();
        self.remove_old("stations".into()).await.unwrap();

        let start = Instant::now();
        for parse_result in parse_results {
            match parse_result {
                ParseResult::TypeIds(x) => self.items(&mut conn, x).await.unwrap(),
                ParseResult::TypeMaterials(x) => self.item_materials(&mut conn, x).await.unwrap(),
                ParseResult::UniqueNames(x) => self.names(&mut conn, x).await.unwrap(),
                ParseResult::Stations(x) => self.stations(&mut conn, x).await.unwrap(),
                _ => (),
            }
        }

        self.metrics
            .set_timing(SdeMetrics::TOTAL_DB_INSERT_TIME, start);
        conn.execute("COMMIT").await.unwrap();

        self.write_checksum().await.unwrap();
        self.metrics.current_time(SdeMetrics::LAST_COMPLETE_READOUT);
        Ok(())
    }

    async fn items(
        &mut self,
        conn: &mut PoolConnection<Postgres>,
        items: HashMap<u32, TypeIds>,
    ) -> Result<(), ()> {
        log::info!("Starting item import");
        let start = Instant::now();

        let mut skip = 0;
        let items_len = items.len();

        let mut ids = Vec::with_capacity(BATCH_SIZE);
        let mut names = Vec::with_capacity(BATCH_SIZE);
        let mut descriptions = Vec::with_capacity(BATCH_SIZE);

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
            }

            sqlx::query(
                r#"INSERT INTO items (id, name, description)
                SELECT * FROM UNNEST($1, $2, $3)
                RETURNING id, name, description"#,
            )
            .bind(&ids)
            .bind(&names)
            .bind(&descriptions)
            .execute(&mut *conn)
            .await
            .unwrap();

            ids.clear();
            names.clear();
            descriptions.clear();
            skip += BATCH_SIZE;
        }

        self.metrics.set_timing(SdeMetrics::ITEM_INSERT_TIME, start);
        log::info!("Importing items done. Took {}s", start.elapsed().as_secs());
        Ok(())
    }

    async fn item_materials(
        &mut self,
        conn: &mut PoolConnection<Postgres>,
        materials: HashMap<u32, TypeMaterial>,
    ) -> Result<(), ()> {
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
            .await
            .unwrap();

            ids.clear();
            material_ids.clear();
            quantities.clear();
            skip += BATCH_SIZE;
        }

        self.metrics
            .set_timing(SdeMetrics::ITEM_MATERIAL_INSERT_TIME, start);
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
    ) -> Result<(), ()> {
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
            .await
            .unwrap();

            ids.clear();
            names_db.clear();
            skip += BATCH_SIZE;
        }

        self.metrics.set_timing(SdeMetrics::NAME_INSERT_TIME, start);
        log::info!("Importing names done. Took {}s", start.elapsed().as_secs());
        Ok(())
    }

    async fn stations(
        &mut self,
        conn: &mut PoolConnection<Postgres>,
        stations: Vec<Station>,
    ) -> Result<(), ()> {
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
            .await
            .unwrap();

            station_ids.clear();
            constellation_ids.clear();
            region_ids.clear();
            solar_system_ids.clear();
            security.clear();
            skip += BATCH_SIZE;
        }

        self.metrics
            .set_timing(SdeMetrics::STATION_INSERT_TIME, start);
        log::info!(
            "Importing stations done. Took {}s",
            start.elapsed().as_secs()
        );
        Ok(())
    }

    async fn remove_old(&mut self, table: String) -> Result<(), ()> {
        let start = Instant::now();
        log::debug!("Removing all unlocked items in {}", table);

        let mut conn = self.db.acquire().await.unwrap();
        sqlx::query(&format!("DELETE FROM {}", table))
            .execute(&mut conn)
            .await
            .unwrap();

        log::debug!("Removed all unlocked items");
        self.metrics.set_timing(SdeMetrics::CLEANUP_TIME, start);
        Ok(())
    }

    async fn has_new_checksum(&self) -> Result<bool, ()> {
        let path = Path::new("sde.checksum");

        let checksum = if path.exists() {
            fs::read_to_string(path).await.unwrap()
        } else {
            String::new()
        };

        if checksum == caph_eve_sde_parser::fetch_checksum().await.unwrap() {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    async fn write_checksum(&self) -> Result<(), ()> {
        let mut file = File::create("sde.checksum").await.unwrap();
        file.write_all(
            caph_eve_sde_parser::fetch_checksum()
                .await
                .unwrap()
                .as_bytes(),
        )
        .await
        .unwrap();
        Ok(())
    }
}
