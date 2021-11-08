use crate::error::CollectorError;

use caph_connector::EveClient;
use caph_connector::services::*;
use sqlx::PgPool;

/// Responsible for getting market data
pub struct Market {
    /// Connection pool to a postgres database
    pool: PgPool,
}

impl Market {
    /// Creates a new instance
    ///
    /// # Params
    ///
    /// * `pool` -> Open connction pool to a postgres
    ///
    /// # Returns
    ///
    /// New instance
    /// 
    pub fn new(pool: PgPool) -> Self {
        Self { pool}
    }

    /// Starts the processing of a sde.zip file
    ///
    /// # Errors
    ///
    /// Fails when there is an error while processing the file
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    pub async fn task(&mut self) -> Result<(), CollectorError> {
        let client = EveClient::new().map_err(CollectorError::CouldNotCreateClient)?;

        let market_service = ConnectMarketService::new(client);

        self.save_market_prices(&market_service).await?;
        self.save_industry_systems(&market_service).await?;

        Ok(())
    }

    /// Gets all market prices and writes them into the database
    ///
    /// # Params
    ///
    /// * `market_service` -> Service that holds information about market prices
    ///
    /// # Errors
    ///
    /// Failes when a database operation fails
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    #[deprecated = "Use caph_core::MarketService::save_market_prices"]
    async fn save_market_prices(&self, market_service: &ConnectMarketService) -> Result<(), CollectorError> {
        let mut type_ids = Vec::new();
        let mut adjusted_prices = Vec::new();
        let mut average_prices = Vec::new();

        tracing::debug!(task = "market", "Loading market information");
        let entries = market_service.market_prices()
            .await
            .map_err(CollectorError::CouldNotGetMarketPrices)?;
        tracing::debug!(task = "market", "Loaded market information");

        tracing::debug!(task = "market", "Start preparing market prices");
        // Collect all items together
        for entry in entries {
            adjusted_prices.push(entry.adjusted_price);
            average_prices.push(entry.average_price);
            type_ids.push(*entry.type_id);
        }
        tracing::debug!(task = "market", "Finsihed preparing market prices");

        tracing::debug!(task = "market", "Start inserting market prices in DB");
        let mut trans = self.pool
            .begin()
            .await
            .map_err(CollectorError::TransactionBeginNotSuccessfull)?;

        sqlx::query!("DELETE FROM market_price")
            .execute(&mut trans)
            .await
            .map_err(CollectorError::DeleteMarketPrices)?;

        sqlx::query!("
                INSERT INTO market_price
                (
                    type_id,
                    adjusted_price,
                    average_price
                )
                SELECT * FROM UNNEST(
                    $1::INTEGER[],
                    $2::DOUBLE PRECISION[],
                    $3::DOUBLE PRECISION[]
                )
            ",
                &type_ids,
                &adjusted_prices,
                &average_prices,
            )
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertMarketPrices)?;

        trans.commit()
            .await
            .map_err(CollectorError::TransactionCommitNotSuccessfull)?;
        tracing::debug!(task = "market", "Finished inserting market prices in DB");

        Ok(())
    }

        /// Gets all market prices and writes them into the database
    ///
    /// # Params
    ///
    /// * `market_service` -> Service that holds information about market prices
    ///
    /// # Errors
    ///
    /// Failes when a database operation fails
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    async fn save_industry_systems(&self, market_service: &ConnectMarketService) -> Result<(), CollectorError> {
        let mut cost_activities = Vec::new();
        let mut cost_indices = Vec::new();
        let mut system_ids = Vec::new();

        tracing::debug!(task = "industry", "Loading industry information");
        let entries = market_service.industry_systems()
            .await
            .map_err(CollectorError::CouldNotGetIndustrySystem)?;
        tracing::debug!(task = "industry", "Loaded industry information");

        tracing::debug!(task = "industry", "Start preparing industry systems");
        // Collect all items together
        for entry in entries {
            for x in entry.cost_indices {
                cost_activities.push(x.activity);
                cost_indices.push(x.cost_index);
                system_ids.push(*entry.solar_system_id);
            }
        }
        tracing::debug!(task = "industry", "Finsihed preparing industry systems");

        tracing::debug!(task = "industry", "Start inserting industry systems in DB");
        let mut trans = self.pool
            .begin()
            .await
            .map_err(CollectorError::TransactionBeginNotSuccessfull)?;

        sqlx::query!("DELETE FROM industry_system")
            .execute(&mut trans)
            .await
            .map_err(CollectorError::DeleteIndustrySystem)?;

        sqlx::query!("
                INSERT INTO industry_system
                (
                    activity,
                    cost_index,
                    system_id
                )
                SELECT * FROM UNNEST(
                    $1::VARCHAR[],
                    $2::DOUBLE PRECISION[],
                    $3::BIGINT[]
                )
            ",
                &cost_activities,
                &cost_indices,
                &system_ids,
            )
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertIndustrySystem)?;

        trans.commit()
            .await
            .map_err(CollectorError::TransactionCommitNotSuccessfull)?;
        tracing::debug!(task = "industry", "Finished inserting industry systems in DB");

        Ok(())
    }
}
