use cachem_utils::{CachemError, Protocol, StorageHandler, cachem};
use carina::*;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec![]);

    let blueprint_cache = Arc::new(BlueprintCache::new().await?);
    let id_name_cache = Arc::new(IdNameCache::new().await?);
    let item_cache = Arc::new(ItemCache::new().await?);
    let item_material_cache = Arc::new(ItemMaterialCache::new().await?);
    let market_order_cache = Arc::new(MarketOrderCache::new().await?);
    let market_order_info_cache = Arc::new(MarketOrderInfoCache::new().await?);
    let region_cache = Arc::new(RegionCache::new().await?);
    let station_cache = Arc::new(StationCache::new().await.unwrap());

    let mut storage_handler = StorageHandler::default();
    storage_handler.register(blueprint_cache.clone());
    storage_handler.register(id_name_cache.clone());
    storage_handler.register(item_cache.clone());
    storage_handler.register(item_material_cache.clone());
    storage_handler.register(market_order_cache.clone());
    storage_handler.register(market_order_info_cache.clone());
    storage_handler.register(region_cache.clone());
    storage_handler.register(station_cache.clone());
    tokio::task::spawn(async move {
        storage_handler.save_on_interrupt().await;
    });

    cachem! { 
        "0.0.0.0:9998",
        let blueprint_copy = blueprint_cache.clone();
        let id_name_copy = id_name_cache.clone();
        let item_copy = item_cache.clone();
        let item_material_copy = item_material_cache.clone();
        let market_order_copy = market_order_cache.clone();
        let market_order_info_copy = market_order_info_cache.clone();
        let region_copy = region_cache.clone();
        let station_copy = station_cache.clone();

        (Action::Fetch, Caches::Blueprint)          => (FetchId, FetchBlueprintEntryById, blueprint_copy),
        (Action::Insert, Caches::Blueprint)         => (Insert, InsertBlueprintEntries, blueprint_copy),

        (Action::Fetch, Caches::IdName)             => (FetchId, FetchNameEntryById, id_name_copy),
        (Action::Insert, Caches::IdName)            => (Insert, InsertIdNameEntries, id_name_copy),

        (Action::Fetch, Caches::Item)               => (FetchId, FetchItemEntryById, item_copy),
        (Action::Insert, Caches::Item)              => (Insert, InsertItemEntries, item_copy),

        (Action::Fetch, Caches::ItemMaterial)       => (FetchId, FetchItemMaterialEntryById, item_material_copy),
        (Action::Insert, Caches::ItemMaterial)      => (Insert, InsertItemMaterialEntries, item_material_copy),

        (Action::Fetch, Caches::MarketOrder)        => (FetchId, FetchMarketOrderEntryById, market_order_copy),
        (Action::Insert, Caches::MarketOrder)       => (Insert, InsertMarketOrderEntries, market_order_copy),

        (Action::Fetch, Caches::MarketOrderInfo)    => (FetchId, FetchMarketOrderInfoEntryById, market_order_info_copy),
        (Action::Lookup, Caches::MarketOrderInfo)   => (Lookup, LookupMarketOrderInfoEntries, market_order_info_copy),
        (Action::Insert, Caches::MarketOrderInfo)   => (Insert, InsertMarketOrderInfoEntries, market_order_info_copy),

        (Action::Fetch, Caches::Region)             => (FetchAll, FetchRegionEntries, region_copy),
        (Action::Insert, Caches::Region)            => (Insert, InsertRegionEntries, region_copy),

        (Action::Fetch, Caches::Station)            => (FetchId, FetchStationEntryById, station_copy),
        (Action::Insert, Caches::Station)           => (Insert, InsertStationEntries, station_copy),
    };
}
