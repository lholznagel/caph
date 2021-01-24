use cachem_utils::{CachemError, Protocol, StorageHandler};
use carina::*;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufStream};
use tokio::net::TcpListener;

macro_rules! s {
    (FetchId, $parse_into:ty, $cache:expr, $socket:expr) => {
        {
            let id = Protocol::read::<_, $parse_into>(&mut $socket).await.unwrap();
            if let Some(x) = $cache.fetch_by_id(id.0).await {
                Protocol::response(&mut $socket, x).await
            } else {
                $socket.write_u8(0u8).await.unwrap();
                Ok(())
            }
        }
    };
    (FetchAll, $parse_into:ty, $cache:expr, $socket:expr) => {
        {
            if let Some(x) = $cache.fetch_all().await {
                Protocol::response(&mut $socket, x).await
            } else {
                $socket.write_u8(0u8).await.unwrap();
                Ok(())
            }
        }
    };
    (Lookup, $parse_from:ty, $cache:expr, $socket:expr) => {
        {
            let data = Protocol::read::<_, $parse_from>(&mut $socket).await.unwrap();
            if let Ok(x) = $cache.lookup(data.0).await {
                Protocol::response(&mut $socket, x).await
            } else {
                Err(CachemError::Empty)
            }
        }
    };
    (Insert, $parse_from:ty, $cache:expr, $socket:expr) => {
        {
            let data = Protocol::read::<_, $parse_from>(&mut $socket).await.unwrap();
            if let Ok(_) = $cache.insert(data.0).await {
                Protocol::response(&mut $socket, EmptyResponse::default()).await
            } else {
                Err(CachemError::Empty)
            }
        }
    };
}

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
    let listener = TcpListener::bind("127.0.0.1:9999").await?;

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

    loop {
        let (mut socket, _) = listener.accept().await?;

        let blueprint_copy = blueprint_cache.clone();
        let id_name_copy = id_name_cache.clone();
        let item_copy = item_cache.clone();
        let item_material_copy = item_material_cache.clone();
        let market_order_copy = market_order_cache.clone();
        let market_order_info_copy = market_order_info_cache.clone();
        let region_copy = region_cache.clone();
        let station_copy = station_cache.clone();
        tokio::spawn(async move {
            // Only read the first two bytes, thats all we need to determine
            // what action and what cache should be used
            let mut buf = [0; 2];

            loop {
                let mut buf_socket = BufStream::new(socket);
                match buf_socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                let action = Action::from(buf[0]);
                let cache = Caches::from(buf[1]);
                let x = match (action, cache) {
                    (Action::Fetch, Caches::Blueprint) => s!(FetchId, FetchBlueprintEntryById, blueprint_copy, buf_socket),
                    (Action::Insert, Caches::Blueprint) => s!(Insert, InsertBlueprintEntries, blueprint_copy, buf_socket),

                    (Action::Fetch, Caches::IdName) => s!(FetchId, FetchNameEntryById, id_name_copy, buf_socket),
                    (Action::Insert, Caches::IdName) => s!(Insert, InsertIdNameEntries, id_name_copy, buf_socket),

                    (Action::Fetch, Caches::Item) => s!(FetchId, FetchItemEntryById, item_copy, buf_socket),
                    (Action::Insert, Caches::Item) => s!(Insert, InsertItemEntries, item_copy, buf_socket),

                    (Action::Fetch, Caches::ItemMaterial) => s!(FetchId, FetchItemMaterialEntryById, item_material_copy, buf_socket),
                    (Action::Insert, Caches::ItemMaterial) => s!(Insert, InsertItemMaterialEntries, item_material_copy, buf_socket),

                    (Action::Fetch, Caches::MarketOrder) => s!(FetchId, FetchMarketOrderEntryById, market_order_copy, buf_socket),
                    (Action::Insert, Caches::MarketOrder) => {
                        {
                            let data = Protocol::read::<_, InsertMarketOrderEntries>(&mut buf_socket).await.unwrap();
                            if let Ok(_) = market_order_copy.insert(data.0).await {
                                Protocol::response(&mut buf_socket, EmptyResponse::default()).await
                            } else {
                                Err(CachemError::Empty)
                            }
                        }
                    }

                    (Action::Fetch, Caches::MarketOrderInfo) => s!(FetchId, FetchMarketOrderInfoEntryById, market_order_info_copy, buf_socket),
                    (Action::Lookup, Caches::MarketOrderInfo) => s!(Lookup, LookupMarketOrderInfoEntries, market_order_info_copy, buf_socket),
                    (Action::Insert, Caches::MarketOrderInfo) => s!(Insert, InsertMarketOrderInfoEntries, market_order_info_copy, buf_socket),

                    (Action::Fetch, Caches::Region) => s!(FetchAll, FetchRegionEntries, region_copy, buf_socket),
                    (Action::Insert, Caches::Region) => s!(Insert, InsertRegionEntries, region_copy, buf_socket),

                    (Action::Fetch, Caches::Station) => s!(FetchId, FetchStationEntryById, station_copy, buf_socket),
                    (Action::Insert, Caches::Station) => s!(Insert, InsertStationEntries, station_copy, buf_socket),
                    _ => panic!("Invalid message {:?}", buf)
                };

                if let Err(e) = x {
                    log::error!("Message error {}", e);
                } else {
                    log::info!("Message ok");
                }
                // return the socket so that we don´t consume it
                socket = buf_socket.into_inner();
            }
        });
    }
}
