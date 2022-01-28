use caph_connector::EveClient;
use caph_core::MarketService;
use caph_server::*;
use sqlx::postgres::PgPoolOptions;
use tracing::Level;
use tracing_subscriber::EnvFilter;

/// ENV variable for the database URL
const PG_ADDR: &'static str = "DATABASE_URL";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    if std::env::var("GIT_HEAD_SHORT").is_ok() {
        tracing_subscriber::fmt()
            .with_max_level(Level::WARN)
            .init();
    } else {
        tracing_subscriber::fmt()
            .pretty()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    }

    let pg_addr = std::env::var(PG_ADDR)
        .expect("Expected that a DATABASE_URL ENV is set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&pg_addr)
        .await?;
    sqlx::migrate!()
        .run(&pool)
        .await?;

    let asset_service     = AssetService::new(pool.clone());
    let auth_service      = AuthService::new(pool.clone());
    let character_service = CharacterService::new(pool.clone());
    let industry_service  = IndustryService::new(pool.clone());
    let item_service      = ItemService::new(pool.clone());
    let project_service   = ProjectService::new(pool.clone(), asset_service.clone());
    let universe_service  = UniverseService::new(pool.clone());

    let eve_client = EveClient::new().map_err(ServerError::ConnectError)?;
    let market_service = MarketService::new(pool.clone(), eve_client.clone());
    let mut project_service2 = caph_core::ProjectService::new(
        pool.clone(),
        market_service.clone()
    );
    project_service2.populate_cache().await?;

    let (tx, rx) = tokio::sync::mpsc::channel(5);
    let market_task = MarketTask::new(market_service, rx);

    let task_service = TaskService::new(tx);

    tokio::task::spawn(async move { market_task.task().await });
    tokio::task::spawn(async move { task_service.task().await });

    start(
        pool,
        asset_service,
        auth_service,
        character_service,
        industry_service,
        item_service,
        project_service,
        universe_service,

        project_service2
    )
    .await?;

    Ok(())
}
