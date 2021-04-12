use caph_eve_sde_parser2::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = SdeServiceLoader::new().await?;
    service.blueprints().await?;
    service.categories().await?;
    service.corporations().await?;
    service.dogma().await?;
    service.groups().await?;
    service.meta_groups().await?;
    service.names().await?;
    service.planet_schematics().await?;
    service.races().await?;
    service.research_agents().await?;
    service.skins().await?;
    service.stations().await?;
    service.type_ids().await?;
    service.type_materials().await?;

    Ok(())
}
