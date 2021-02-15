use cachem::{CachemError, Protocol, Fetch, Insert, Storage, cachem};
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

        (Actions::Fetch, Caches::Blueprint)          => (blueprint_copy, fetch, FetchBlueprintEntryById),
        (Actions::Insert, Caches::Blueprint)         => (blueprint_copy, insert, InsertBlueprintEntries),

        (Actions::Fetch, Caches::IdName)             => (id_name_copy, fetch, FetchNameEntryById),
        (Actions::Insert, Caches::IdName)            => (id_name_copy, insert, InsertIdNameEntries),

        (Actions::Fetch, Caches::Item)               => (item_copy, fetch, FetchItemEntryById),
        (Actions::Insert, Caches::Item)              => (item_copy, insert, InsertItemEntries),

        (Actions::Fetch, Caches::ItemMaterial)       => (item_material_copy, fetch, FetchItemMaterialEntryById),
        (Actions::Insert, Caches::ItemMaterial)      => (item_material_copy, insert, InsertItemMaterialEntries),

        (Actions::Fetch, Caches::MarketOrder)        => (market_order_copy, fetch, FetchMarketOrderEntries),
        (Actions::Insert, Caches::MarketOrder)       => (market_order_copy, insert, InsertMarketOrderEntries),

        (Actions::Fetch, Caches::MarketOrderInfo)    => (market_order_info_copy, fetch, FetchMarketOrderInfoReq),
        (Actions::Insert, Caches::MarketOrderInfo)   => (market_order_info_copy, insert, InsertMarketOrderInfoReq),

        (Actions::Fetch, Caches::Region)             => (region_copy, fetch, FetchRegionEntries),
        (Actions::Insert, Caches::Region)            => (region_copy, insert, InsertRegionEntries),

        (Actions::Fetch, Caches::Station)            => (station_copy, fetch, FetchStationEntryById),
        (Actions::Insert, Caches::Station)           => (station_copy, insert, InsertStationEntries),
    };
}
