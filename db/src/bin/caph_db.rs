use cachem::*;
use caph_db::*;

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

    load_and_register!(CacheName::AllianceFitting,      AllianceFittingCache,      cnc, server);
    load_and_register!(CacheName::Blueprint,            BlueprintCache,            cnc, server);
    load_and_register!(CacheName::CharacterAsset,       CharacterAssetCache,       cnc, server);
    load_and_register!(CacheName::CharacterBlueprint,   CharacterBlueprintCache,   cnc, server);
    load_and_register!(CacheName::CharacterFitting,     CharacterFittingCache,     cnc, server);
    load_and_register!(CacheName::CorporationBlueprint, CorporationBlueprintCache, cnc, server);
    load_and_register!(CacheName::IndustryCost,         IndustryCostCache,         cnc, server);
    load_and_register!(CacheName::Item,                 ItemCache,                 cnc, server);
    load_and_register!(CacheName::ItemDogma,            ItemDogmaCache,            cnc, server);
    load_and_register!(CacheName::Login,                LoginCache,                cnc, server);
    load_and_register!(CacheName::Name,                 NameCache,                 cnc, server);
    load_and_register!(CacheName::Project,              ProjectCache,              cnc, server);
    load_and_register!(CacheName::MarketPrice,          MarketPriceCache,          cnc, server);
    load_and_register!(CacheName::Reprocess,            ReprocessCache,            cnc, server);
    load_and_register!(CacheName::Schematic,            SchematicCache,            cnc, server);
    load_and_register!(CacheName::SystemRegion,         SystemRegionCache,         cnc, server);
    load_and_register!(CacheName::User,                 UserCache,                 cnc, server);

    server.listen_tcp().await;

    Ok(())
}
