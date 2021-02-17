use cachem::{Protocol, Fetch, Insert, Storage, cachem};
use caph_db::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec![]);

    let blueprint_cache = Arc::new(BlueprintCache::default());
    let id_name_cache = Arc::new(IdNameCache::default());
    let item_cache = Arc::new(ItemCache::default());
    let item_material_cache = Arc::new(ItemMaterialCache::default());
    let region_cache = Arc::new(RegionCache::default());
    let station_cache = Arc::new(StationCache::default());
    
    let market_order_info_cache = Arc::new(MarketOrderInfoCache::load_from_file().await?);
    let market_order_cache = Arc::new(MarketOrderCache::load_from_file().await?);

    cachem! {
        "0.0.0.0:9999",
        let blueprint_copy = blueprint_cache.clone();
        let id_name_copy = id_name_cache.clone();
        let item_copy = item_cache.clone();
        let item_material_copy = item_material_cache.clone();
        let market_order_copy = market_order_cache.clone();
        let market_order_info_copy = market_order_info_cache.clone();
        let region_copy = region_cache.clone();
        let station_copy = station_cache.clone();

        - Actions::InsertBlueprints       => (blueprint_copy, insert, InsertBlueprintReq),

        - Actions::InsertIdNames          => (id_name_copy, insert, InsertIdNameReq),

        - Actions::InsertItems            => (item_copy, insert, InsertItemReq),

        - Actions::InsertItemMaterials    => (item_material_copy, insert, InsertItemMaterialReq),

        - Actions::FetchMarketOrder       => (market_order_copy, fetch, FetchMarketOrderReq),
        - Actions::InsertMarketOrders     => (market_order_copy, insert, InsertMarketOrderReq),

        - Actions::FetchMarketOrderInfo   => (market_order_info_copy, fetch, FetchMarketOrderInfoReq),
        - Actions::InsertMarketOrdersInfo => (market_order_info_copy, insert, InsertMarketOrderInfoReq),

        - Actions::FetchRegions           => (region_copy, fetch, FetchRegionReq),
        - Actions::InsertRegions          => (region_copy, insert, InsertRegionReq),

        - Actions::InsertStations         => (station_copy, insert, InsertStationReq),
    };
}
