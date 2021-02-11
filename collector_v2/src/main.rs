mod error;
mod market;
mod sde;
mod time;

use self::market::*;
use self::sde::*;
use self::time::*;

use cachem::ConnectionPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec![]);

    let pool = ConnectionPool::new("127.0.0.1:9999".into(), 10).await?;

    let mut sde = Sde::new(pool.clone()).await;
    sde.run().await.unwrap();

    let pool_copy = pool.clone();
    tokio::task::spawn(async {
        let mut market = Market::new(pool_copy);

        loop {
            if let Err(e) = market.task().await {
                log::error!("Error running market task {:?}", e);
            }

            let next_run = duration_to_next_30_minute();
            tokio::time::sleep(next_run).await; // Run on the next 30 minute interval
        }
    })
    .await
    .unwrap();

    /*
    #[allow(unused_must_use)]
    {
        tokio::join!(
            market,
        );
    }*/

    Ok(())
}
