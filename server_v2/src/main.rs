use server_v2::*;
use sqlx::postgres::PgPoolOptions;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

/// ENV variable for the database URL
const PG_ADDR: &'static str = "DATABASE_URL";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let pg_addr = std::env::var(PG_ADDR)
        .expect("Expected that a DATABASE_URL ENV is set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&pg_addr)
        .await?;

    let asset_service = AssetService::new(pool.clone());
    let character_service = CharacterService::new(pool.clone());
    let eve_service = EveService::new(pool.clone());

    start(
        asset_service,
        character_service,
        eve_service
    )
    .await?;

    Ok(())
}
