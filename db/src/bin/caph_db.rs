use cachem::{Protocol, Fetch, Insert, Storage, cachem};
use caph_db::*;
use metrix_exporter::Metrix;
use std::path::Path;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec![]);

    let metrix = Metrix::new(env!("CARGO_PKG_NAME").into(), "0.0.0.0:8889").await?;

    if !Path::new("./db/storage").exists() {
        std::fs::create_dir_all("./db/storage")?;
    }

    let blueprint_cache = BlueprintCache::new(metrix.get_sender());
    blueprint_cache.load_from_file().await?;
    let blueprint_cache = Arc::new(blueprint_cache);

    let item_cache = ItemCache::new(metrix.get_sender());
    item_cache.load_from_file().await?;
    let item_cache = Arc::new(item_cache);

    let item_material_cache = ItemMaterialCache::new(metrix.get_sender());
    item_material_cache.load_from_file().await?;
    let item_material_cache = Arc::new(item_material_cache);

    let market_order_cache = MarketOrderCache::new(metrix.get_sender());
    market_order_cache.load_from_file().await?;
    let market_order_cache = Arc::new(market_order_cache);

    let market_order_info_cache = MarketOrderInfoCache::new(metrix.get_sender());
    market_order_info_cache.load_from_file().await?;
    let market_order_info_cache = Arc::new(market_order_info_cache);

    let station_cache = StationCache::new(metrix.get_sender());
    station_cache.load_from_file().await?;
    let station_cache = Arc::new(station_cache);

    let user_cache = UserCache::new(metrix.get_sender());
    user_cache.load_from_file().await?;
    let user_cache = Arc::new(user_cache);

    tokio::task::spawn(async move {
        metrix.listen().await
    });

    cachem! {
        "0.0.0.0:9999",

        let blueprint_copy = blueprint_cache.clone();
        let item_copy = item_cache.clone();
        let item_material_copy = item_material_cache.clone();
        let market_order_copy = market_order_cache.clone();
        let market_order_info_copy = market_order_info_cache.clone();
        let station_copy = station_cache.clone();
        let user_copy = user_cache.clone();

        - Actions::FetchBlueprint           => (blueprint_copy, fetch, FetchBlueprintReq),
        - Actions::InsertBlueprints         => (blueprint_copy, insert, InsertBlueprintReq),

        - Actions::FetchItem                => (item_copy, fetch, FetchItemReq),
        - Actions::InsertItems              => (item_copy, insert, InsertItemReq),

        - Actions::FetchItemMaterial        => (item_material_copy, fetch, FetchItemMaterialReq),
        - Actions::InsertItemMaterials      => (item_material_copy, insert, InsertItemMaterialReq),

        - Actions::FetchMarketOrder         => (market_order_copy, fetch, FetchMarketOrderReq),
        - Actions::FetchMarketOrderItemIds  => (market_order_copy, fetch, FetchMarketOrderItemIdsReq),
        - Actions::FetchLatestMarketOrders  => (market_order_copy, fetch, FetchLatestMarketOrdersReq),
        - Actions::InsertMarketOrders       => (market_order_copy, insert, InsertMarketOrderReq),

        - Actions::FetchMarketOrderInfo     => (market_order_info_copy, fetch, FetchMarketOrderInfoReq),
        - Actions::FetchMarketOrderInfoBulk => (market_order_info_copy, fetch, FetchMarketOrderInfoBulkReq),
        - Actions::InsertMarketOrdersInfo   => (market_order_info_copy, insert, InsertMarketOrderInfoReq),

        - Actions::FetchStation             => (station_copy, fetch, FetchStationReq),
        - Actions::InsertStations           => (station_copy, insert, InsertStationReq),

        - Actions::FetchUser                => (user_copy, fetch, FetchUserReq),
        - Actions::InsertUser               => (user_copy, insert, InsertUserReq),
    };
}
