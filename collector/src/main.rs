mod error;
mod market;
mod metrics;
mod postgres;
mod sde;

use metrix::{MetricCollector, MetricCommand, Metrics};
use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;
use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init()?;

    let (metric_tx, metric_rx) = mpsc::channel::<(&str, MetricCommand)>(100);
    let metric_collector = MetricCollector::default();

    let db_uri =
        std::env::var("DB").unwrap_or("postgres://caph:caph@cygnus.local:5432/caph_test".into());
    let pool = PgPoolOptions::new()
        .max_connections(25)
        .connect(&db_uri)
        .await?;

    // Make sure the database has the newest scripts applied
    let mut conn = pool.acquire().await?;
    conn.execute(include_str!("./tables.sql")).await?;

    let pool_copy = pool.clone();
    let metric_tx_copy = metric_tx.clone();
    let sde = tokio::task::spawn(async {
        let metrics = Metrics::new(metric_tx_copy);
        let mut sde = sde::Sde::new(pool_copy, metrics).await;

        loop {
            if let Err(e) = sde.background().await {
                log::error!("Error running SDE task: {:?}", e);
            }

            tokio::time::delay_for(Duration::from_secs(24 * 60 * 60)).await; // 24 hours
        }
    });

    let pool_copy = pool.clone();
    let metric_tx_copy = metric_tx.clone();
    let market = tokio::task::spawn(async {
        let metrics = Metrics::new(metric_tx_copy);
        let mut market = market::Market::new(pool_copy, metrics);

        loop {
            if let Err(e) = market.background().await {
                log::error!("Error running market task {:?}", e);
            }

            tokio::time::delay_for(Duration::from_secs(30 * 60)).await; // 30 minutes
        }
    });

    let pool_copy = pool.clone();
    let metric_tx_copy = metric_tx.clone();
    let postgres = tokio::task::spawn(async {
        let metrics = Metrics::new(metric_tx_copy);
        let mut postgres = postgres::PostgresService::new(pool_copy, metrics);

        loop {
            postgres.background().await;
            tokio::time::delay_for(Duration::from_secs(15 * 60)).await; // 15 minutes
        }
    });

    #[allow(unused_must_use)]
    {
        tokio::join!(
            sde,
            market,
            postgres,
            metric_collector.metric_server("127.0.0.1:9000"),
            metric_collector.background(metric_rx),
        );
    }

    Ok(())
}
