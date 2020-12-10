use metrics_exporter_http::HttpExporter;
use metrics_runtime::{observers::PrometheusBuilder, Controller, Receiver, Sink};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::error::CollectorError;

pub struct Metrics {
    pub market: MarketMetrics,
    pub postgres: PostgresMetrics,
    pub sde: SdeMetrics,

    controller: Controller,
}

impl Metrics {
    pub async fn task(&self) -> Result<(), CollectorError> {
        let exporter = HttpExporter::new(
            self.controller.clone(),
            PrometheusBuilder::new(),
            "0.0.0.0:9000".parse().map_err(CollectorError::ParseError)?,
        );

        tokio::task::spawn(async move {
            if let Err(e) = exporter.async_run().await {
                log::error!("Error starting metric http exporter {:?}", e);
            }
        });

        Ok(())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        let receiver = Receiver::builder().build().unwrap();
        let sink = receiver.sink();

        Self {
            market: MarketMetrics::new(sink.clone()),
            postgres: PostgresMetrics::new(sink.clone()),
            sde: SdeMetrics::new(sink.clone()),

            controller: receiver.controller(),
        }
    }
}

#[derive(Clone)]
pub struct MarketMetrics {
    sink: Sink,
}

impl MarketMetrics {
    pub const LAST_COMPLETE_READOUT: &'static str = "market_last_readout";
    pub const EVE_DOWNLOAD_TIME: &'static str = "market_eve_download_time";
    pub const TOTAL_DB_INSERT_TIME: &'static str = "market_total_db_insert_time";
    pub const MARKET_INFO_INSERT_TIME: &'static str = "market_info_insert_time";
    pub const MARKET_HISTORY_INSERT_TIME: &'static str = "market_history_insert_time";
    pub const CLEANUP_TIME: &'static str = "market_cleanup_time";

    pub fn new(sink: Sink) -> Self {
        Self { sink }
    }

    pub fn set_timing(&mut self, name: &'static str, time: Instant) {
        self.sink
            .record_timing(name, 0, time.elapsed().as_millis() as u64);
    }

    pub fn current_time(&mut self, name: &'static str) -> Result<(), CollectorError> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| CollectorError::ClockRunsBackwards)?;
        self.sink.update_gauge(name, timestamp.as_millis() as i64);
        Ok(())
    }
}

#[derive(Clone)]
pub struct SdeMetrics {
    sink: Sink,
}

impl SdeMetrics {
    // Time of date since 1970 in milliseconds
    pub const LAST_COMPLETE_READOUT: &'static str = "sde_last_readout";

    // Duration in milliseconds from start to finish
    pub const DOWNLOAD_TIME: &'static str = "sde_ownload_time";
    pub const PARSE_TIME: &'static str = "sde_parse_time";
    pub const TOTAL_DB_INSERT_TIME: &'static str = "sde_total_db_insert_time";
    pub const ITEM_INSERT_TIME: &'static str = "sde_item_insert_time";
    pub const ITEM_MATERIAL_INSERT_TIME: &'static str = "sde_item_masterial_insert_time";
    pub const NAME_INSERT_TIME: &'static str = "sde_name_insert_time";
    pub const STATION_INSERT_TIME: &'static str = "sde_station_insert_time";
    pub const BLUEPRINT_INSERT_TIME: &'static str = "sde_blueprint_insert_time";
    pub const SCHEMATIC_INSERT_TIME: &'static str = "sde_schematic_insert_time";
    pub const CLEANUP_TIME: &'static str = "sde_cleanup_time";

    pub fn new(sink: Sink) -> Self {
        Self { sink }
    }

    pub fn set_timing(&mut self, name: &'static str, time: Instant) {
        self.sink
            .record_timing(name, 0, time.elapsed().as_millis() as u64);
    }

    pub fn current_time(&mut self, name: &'static str) -> Result<(), CollectorError> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| CollectorError::ClockRunsBackwards)?;
        self.sink.update_gauge(name, timestamp.as_millis() as i64);
        Ok(())
    }
}

#[derive(Clone)]
pub struct PostgresMetrics {
    sink: Sink,
}

impl PostgresMetrics {
    pub const TABLE_ITEMS: &'static str = "postgres_items";
    pub const TABLE_ITEM_MATERIALS: &'static str = "postgres_item_materials";
    pub const TABLE_NAMES: &'static str = "postgres_names";
    pub const TABLE_STATIONS: &'static str = "postgres_stations";
    pub const TABLE_BLUEPRINTS: &'static str = "postgres_blueprints";
    pub const TABLE_BLUEPRINT_RESOURCES: &'static str = "postgres_blueprint_resources";
    pub const TABLE_SCHEMATICS: &'static str = "postgres_schematics";
    pub const TABLE_SCHEMATIC_RESOURCES: &'static str = "postgres_schematic_resources";
    pub const TABLE_MARKET: &'static str = "postgres_market";
    pub const TABLE_MARKET_ORDER_INFO: &'static str = "postgres_market_order_info";
    pub const TABLE_MARKET_HISTORY: &'static str = "postgres_market_history";

    pub fn new(sink: Sink) -> Self {
        Self { sink }
    }

    pub fn set(&mut self, name: &'static str, count: i64, size: i64) {
        self.sink.update_gauge(format!("{}_count", name), count);
        self.sink.update_gauge(format!("{}_size", name), size);
    }
}
