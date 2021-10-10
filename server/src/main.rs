use caph_server::*;
use sqlx::postgres::PgPoolOptions;
use tracing::Level;

/// ENV variable for the database URL
const PG_ADDR: &'static str = "DATABASE_URL";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(Level::INFO)
        .init();

    let pg_addr = std::env::var(PG_ADDR)
        .expect("Expected that a DATABASE_URL ENV is set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&pg_addr)
        .await?;

    let asset_service     = AssetService::new(pool.clone());
    let character_service = CharacterService::new(pool.clone());
    let eve_service       = EveService::new(pool.clone());
    let industry_service  = IndustryService::new(pool.clone());
    let item_service      = ItemService::new(pool.clone());
    let project_service   = ProjectService::new(pool.clone(), asset_service.clone());
    let universe_service  = UniverseService::new(pool.clone());

    start(
        asset_service,
        character_service,
        eve_service,
        industry_service,
        item_service,
        project_service,
        universe_service,
    )
    .await?;

    Ok(())
}
