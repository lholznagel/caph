use caph_core::MarketService;
use tokio::sync::mpsc::Receiver;

/// Datatype of the required MPSC channel
pub type MarketMpscType = bool;

/// Struct for running the fetching of marketing information in a background
/// task
pub struct MarketTask {
    /// Market API wrapper
    market: MarketService,
    /// Receiver for triggering the task
    rx:     Receiver<MarketMpscType>
}

impl MarketTask {
    /// Creates a new task instance.
    ///
    /// # Params
    ///
    /// * `market` -> Wrapper for the market
    /// * `rx`     -> Receiver part of an mpsc channel
    ///
    /// # Returns
    ///
    /// New task instance.
    ///
    pub fn new(
        market: MarketService,
        rx:     Receiver<MarketMpscType>
    ) -> Self {
        Self {
            market,
            rx
        }
    }

    /// Starts the task in the background.
    ///
    /// Warning: This function is blocking!
    ///
    pub async fn task(mut self) {
        while let Some(true) = self.rx.recv().await {
            tracing::info!("Starting market task.");
            let r = self.market.save_market_orders().await;
            if let Err(e) = r {
                tracing::error!("Error running market task. {:?}", e);
            }
            tracing::info!("Finished market task.");
        }
    }
}
