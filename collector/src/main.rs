use sqlx::postgres::PgPoolOptions;

/// ENV variable for the database URL
const PG_ADDR: &str = "DATABASE_URL";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let pg_addr = std::env::var(PG_ADDR).expect("Expected that a DATABASE_URL ENV is set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&pg_addr)
        .await?;
    sqlx::migrate!().run(&pool).await?;

    Ok(())
}
