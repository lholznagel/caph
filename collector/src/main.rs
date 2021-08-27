mod character;
mod error;
mod sde;
mod time;

use self::character::*;
use self::sde::*;
use self::time::*;

use cachem::ConnectionPool;
use caph_eve_data_wrapper::EveDataWrapper;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec![]);

    let pool = ConnectionPool::new("0.0.0.0:55555", 10usize).await?;

    log::info!("Preparing SDE");
    let eve = EveDataWrapper::new().await?;
    log::info!("Prepared SDE");

    let eve_copy = eve.clone();
    let pool_copy = pool.clone();
    let sde = tokio::task::spawn(async {
        let mut sde = Sde::new(eve_copy, pool_copy);

        loop {
            log::info!("SDE start");
            if let Err(e) = sde.run().await {
                log::error!("Error running sde task {:?}", e);
            }
            log::info!("SDE done");

            let next_run = duration_next_sde_download()
                .unwrap_or_else(|_| Duration::from_secs(24 * 60 * 60));
            tokio::time::sleep(next_run).await;
        }
    });

    let eve_copy = eve.clone();
    let pool_copy = pool.clone();
    let character = tokio::task::spawn(async {
        let mut market = Character::new(eve_copy, pool_copy);

        loop {
            log::info!("Character start");
            if let Err(e) = market.task().await {
                log::error!("Error running market task {:?}", e);
            }
            log::info!("Character done");

            let next_run = duration_to_next_10_minute()
                .unwrap_or_else(|_| Duration::from_secs(30 * 60));
            tokio::time::sleep(next_run).await; // Run on the next 30 minute interval
        }
    });

    let _ = tokio::join!(
        character,
        sde,
    );

    Ok(())
}
