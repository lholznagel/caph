mod error;
mod market;
mod sde;

use self::market::*;
use self::sde::*;

use cachem::ConnectionPool;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec![]);

    let pool = ConnectionPool::new("127.0.0.1:9999".into(), 10).await?;

    let pool_copy = pool.clone();
    let market = tokio::task::spawn(async {
        let mut market = Market::new(pool_copy);

        loop {
            if let Err(e) = market.background().await {
                log::error!("Error running market task {:?}", e);
            }

            tokio::time::sleep(Duration::from_secs(15 * 60)).await; // 15 minutes
        }
    });

    let pool_copy = pool.clone();
    let sde = tokio::task::spawn(async {
        let mut sde = Sde::new(pool_copy).await;

        loop {
            if let Err(e) = sde.background().await {
                log::error!("Error running SDE task: {:?}", e);
            }

            tokio::time::sleep(Duration::from_secs(24 * 60 * 60)).await; // 24 hours
        }
    });

    #[allow(unused_must_use)]
    {
        tokio::join!(
            market,
            sde,
        );
    }

    Ok(())
}
