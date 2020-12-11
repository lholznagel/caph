pub struct MarketMetrics;
impl MarketMetrics {
    pub const LAST_COMPLETE_READOUT: &'static str = "market_last_readout";
    pub const EVE_DOWNLOAD_TIME: &'static str = "market_eve_download_time";
    pub const TOTAL_DB_INSERT_TIME: &'static str = "market_total_db_insert_time";
    pub const MARKET_INFO_INSERT_TIME: &'static str = "market_info_insert_time";
    pub const MARKET_HISTORY_INSERT_TIME: &'static str = "market_history_insert_time";
    pub const CLEANUP_TIME: &'static str = "market_cleanup_time";
}

pub struct SdeMetrics;
impl SdeMetrics {
    pub const LAST_COMPLETE_READOUT: &'static str = "sde_last_readout";

    pub const DOWNLOAD_TIME: &'static str = "sde_download_time";
    pub const PARSE_TIME: &'static str = "sde_parse_time";
    pub const TOTAL_DB_INSERT_TIME: &'static str = "sde_total_db_insert_time";
    pub const ITEM_INSERT_TIME: &'static str = "sde_item_insert_time";
    pub const ITEM_MATERIAL_INSERT_TIME: &'static str = "sde_item_masterial_insert_time";
    pub const NAME_INSERT_TIME: &'static str = "sde_name_insert_time";
    pub const STATION_INSERT_TIME: &'static str = "sde_station_insert_time";
    pub const BLUEPRINT_INSERT_TIME: &'static str = "sde_blueprint_insert_time";
    pub const SCHEMATIC_INSERT_TIME: &'static str = "sde_schematic_insert_time";
    pub const CLEANUP_TIME: &'static str = "sde_cleanup_time";
}

pub struct PostgresMetrics;
impl PostgresMetrics {
    pub const TABLE_ITEMS_COUNT: &'static str = "postgres_items_count";
    pub const TABLE_ITEMS_SIZE: &'static str = "postgres_items_size";

    pub const TABLE_ITEM_MATERIALS_COUNT: &'static str = "postgres_item_materials_count";
    pub const TABLE_ITEM_MATERIALS_SIZE: &'static str = "postgres_item_materials_size";

    pub const TABLE_NAMES_COUNT: &'static str = "postgres_names_count";
    pub const TABLE_NAMES_SIZE: &'static str = "postgres_names_size";

    pub const TABLE_STATIONS_COUNT: &'static str = "postgres_stations_count";
    pub const TABLE_STATIONS_SIZE: &'static str = "postgres_stations_size";

    pub const TABLE_BLUEPRINTS_COUNT: &'static str = "postgres_blueprints_count";
    pub const TABLE_BLUEPRINTS_SIZE: &'static str = "postgres_blueprints_size";

    pub const TABLE_BLUEPRINT_RESOURCES_COUNT: &'static str = "postgres_blueprint_resources_count";
    pub const TABLE_BLUEPRINT_RESOURCES_SIZE: &'static str = "postgres_blueprint_resources_size";

    pub const TABLE_SCHEMATICS_COUNT: &'static str = "postgres_schematics_count";
    pub const TABLE_SCHEMATICS_SIZE: &'static str = "postgres_schematics_size";

    pub const TABLE_SCHEMATIC_RESOURCES_COUNT: &'static str = "postgres_schematic_resources_count";
    pub const TABLE_SCHEMATIC_RESOURCES_SIZE: &'static str = "postgres_schematic_resources_size";

    pub const TABLE_MARKET_CURRENT_COUNT: &'static str = "postgres_market_current_size";
    pub const TABLE_MARKET_CURRENT_SIZE: &'static str = "postgres_market_current_count";

    pub const TABLE_MARKET_ORDERS_COUNT: &'static str = "postgres_market_orders_count";
    pub const TABLE_MARKET_ORDERS_SIZE: &'static str = "postgres_market_orders_size";

    pub const TABLE_MARKET_HISTORY_COUNT: &'static str = "postgres_market_history_count";
    pub const TABLE_MARKET_HISTORY_SIZE: &'static str = "postgres_market_history_size";
}
