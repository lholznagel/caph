use crate::Time;

use tokio::sync::mpsc::Sender;

/// Handles all background tasks
pub struct TaskService {
    tx_market: Sender<crate::MarketMpscType>
}

impl TaskService {
    /// Creates a new task service.
    /// Requires the [Sender] parts of the mpsc channels
    ///
    /// # Params
    ///
    /// * `tx_market` -> Sender part for the market task
    ///
    /// # Returns
    ///
    /// New instance.
    ///
    pub fn new(
        tx_market: Sender<crate::MarketMpscType>
    ) -> Self {
        Self {
            tx_market
        }
    }

    /// Starts the background task and triggers other background tasks
    ///
    /// Warning: This task is blocking!
    ///
    pub async fn task(self) {
        let tx_market = self.tx_market.clone();
        let _ = tokio::task::spawn(async move {
            loop {
                // ignore the error, it will restart after a few minutes
                if let Err(e) = tx_market.send(true).await {
                    tracing::error!("Error executing market background task. {}", e);
                }
                tokio::time::sleep(Time::default().duration_next_market()).await;
            }
        }).await;
    }
}
