use cachem::v2::*;
use caph_db_v2::*;

macro_rules! load_and_register {
    ($name:path, $cache:ident, $cnc:ident, $server:ident) => {
        let x = $cache::new($cnc.clone());
        x.load().await;
        $server.add($name, x.into());
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (cnc, mut server) = Server::new("0.0.0.0:55555".into());

    let market_info = MarketInfoCache::new(cnc.clone());
    //market_info.load().await;

    let market_order = MarketOrderCache::new(cnc.clone(), market_info.clone());
    //market_order.load().await;

    server.add(CacheName::MarketInfo, market_info.clone().into());
    server.add(CacheName::MarketOrder, market_order.into());

    load_and_register!(CacheName::Blueprint,    BlueprintCache,    cnc, server);
    load_and_register!(CacheName::Item,         ItemCache,         cnc, server);
    load_and_register!(CacheName::Name,         NameCache,         cnc, server);
    load_and_register!(CacheName::Reprocess,    ReprocessCache,    cnc, server);
    load_and_register!(CacheName::Schematic,    SchematicCache,    cnc, server);
    load_and_register!(CacheName::SystemRegion, SystemRegionCache, cnc, server);
    load_and_register!(CacheName::User,         UserCache,         cnc, server);

    server.listen_tcp().await;

    Ok(())
}
