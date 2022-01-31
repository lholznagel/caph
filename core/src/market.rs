use caph_connector::{EveClient, RegionId, RequestClient, SystemId, TypeId};
use futures::StreamExt;
use futures::stream::FuturesUnordered;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;

/// Wrapps the market EVE-API
#[derive(Clone)]
pub struct MarketService {
    /// Database pool
    pool:   PgPool,
    /// Client for the EVE-API
    client: EveClient
}

impl MarketService {
    /// Path to the market prices
    const PATH_MARKET_PRICES: &'static str = "v1/markets/prices";
    const PATH_REGIONS: &'static str       = "v1/universe/regions";

    /// Creates a new service instance.
    ///
    /// # Params
    ///
    /// * `pool`       -> Postgres Database pool
    /// * `eve_client` -> Unauthorized eve client
    ///
    /// # Returns
    ///
    /// New service instance.
    ///
    pub fn new(
        pool:   PgPool,
        client: EveClient
    ) -> Self {
        Self {
            pool,
            client
        }
    }

    /// Gets the min, max and average cost of an [TypeId] for the given
    /// [SystemId].
    ///
    /// # Params
    ///
    /// * `tid` -> [TypeId] to fetch the prices from
    /// * `sid` -> [SystemId] of the system to fetch the prices from
    ///
    /// # Errors
    ///
    /// If the database access fails.
    ///
    /// # Returns
    ///
    /// Min, max and average prices for the item_cost
    ///
    pub async fn item_cost_bulk(
        &self,
        tid:    Vec<TypeId>,
        sid:    SystemId,
        is_buy: bool
    ) -> Result<Vec<MarketItemPrice>, MarketError> {
        let tid = tid.into_iter().map(|x| *x).collect::<Vec<_>>();
        let entries = sqlx::query!(r#"
                SELECT
                    AVG(mo.price) AS "avg!",
                    MIN(mo.price) AS "min!",
                    MAX(mo.price) AS "max!",
                    mo.type_id    AS "type_id!",
                    i.name        AS "name!"
                FROM market_orders mo
                JOIN items i
                  ON i.type_id = mo.type_id
                WHERE mo.type_id = ANY($1)
                  AND mo.system_id = $2
                  AND mo.is_buy_order = $3
                GROUP BY mo.type_id, i.name, i.group_id
                ORDER BY i.group_id ASC
            "#,
                &tid,
                *sid,
                is_buy
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| MarketItemPrice {
                min:     x.min,
                max:     x.max,
                avg:     x.avg,
                type_id: x.type_id.into(),
                name:    x.name
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Fetches all orders for the given [RegionId].
    /// Wrapper for `/v1/markets/{region_id}/orders`.
    ///
    /// # Params
    ///
    /// * `rid` -> [RegionId] to fetch for orders
    ///
    /// # Errors
    ///
    /// If the API call to EVE fails.
    ///
    /// # Returns
    ///
    /// List of all orders for the given [RegionId].
    ///
    #[instrument(err)]
    pub async fn fetch_market_orders(
        &self,
        rid: RegionId
    ) -> Result<Vec<MarketOrder>, MarketError> {
        let path = format!("v1/markets/{}/orders", rid);
        self
            .client
            .fetch_page::<MarketOrder>(&path)
            .await
            .map_err(Into::into)
    }

    /// Fetches and inserts all market orders for every region in the database.
    /// The data is fetched from the EVE-API.
    ///
    /// # Errors
    ///
    /// If either the API requests or the database access fails.
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    #[instrument(err)]
    pub async fn save_market_orders(
        &self,
    ) -> Result<(), MarketError> {
        let mut trans = self.pool.begin().await?;
        let mut tasks = FuturesUnordered::new();

        sqlx::query!("DELETE FROM market_orders")
            .execute(&mut trans)
            .await?;

        let regions = self.fetch_regions().await?;
        for rid in regions {
            tasks.push(self.fetch_market_orders(rid));
        }

        while let Some(e) = tasks.next().await {
            let entries = e?;

            let is_buy_orders = entries
                .iter()
                .map(|x| x.is_buy_order)
                .collect::<Vec<_>>();
            let type_ids = entries
                .iter()
                .map(|x| *x.type_id)
                .collect::<Vec<_>>();
            let system_ids = entries
                .iter()
                .map(|x| *x.system_id)
                .collect::<Vec<_>>();
            let price = entries
                .iter()
                .map(|x| x.price)
                .collect::<Vec<_>>();

            sqlx::query!("
                    INSERT INTO market_orders
                    (
                        is_buy_order,
                        type_id,
                        system_id,
                        price
                    )
                    SELECT * FROM UNNEST(
                        $1::BOOLEAN[],
                        $2::INTEGER[],
                        $3::BIGINT[],
                        $4::DOUBLE PRECISION[]
                    )
                ",
                    &is_buy_orders,
                    &type_ids,
                    &system_ids,
                    &price
                )
                .execute(&mut trans)
                .await?;
        }

        trans.commit().await?;

        Ok(())
    }

    /// Fetches and inserts all market prices into the database.
    /// The data is fetched from the EVE-API.
    ///
    /// # Errors
    ///
    /// If the database is not available.
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    #[instrument(err)]
    pub async fn save_market_prices(
        &self
    ) -> Result<(), MarketError> {
        let mut trans = self.pool.begin().await?;

        let entries = self.fetch_market_prices().await?;

        let type_ids = entries
            .iter()
            .map(|x| *x.type_id)
            .collect::<Vec<_>>();
        let adj_prices = entries
            .iter()
            .map(|x| x.adjusted_price)
            .collect::<Vec<_>>();
        let avg_prices = entries
            .iter()
            .map(|x| x.average_price)
            .collect::<Vec<_>>();

        sqlx::query!("DELETE FROM market_prices")
            .execute(&mut trans)
            .await?;
        sqlx::query!("
                INSERT INTO market_prices
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
                &adj_prices,
                &avg_prices
            )
            .execute(&mut trans)
            .await?;

        trans.commit().await?;

        Ok(())
    }

    /// Fetches the price of all items that are in the game from the EVE-API.
    /// Wrapper for `/v1/markets/prices`.
    ///
    /// # Returns
    ///
    /// List of all items and their adjusted and average price.
    ///
    #[instrument(err)]
    pub async fn fetch_market_prices(
        &self
    ) -> Result<Vec<MarketPrice>, MarketError> {
        self
            .client
            .fetch::<Vec<MarketPrice>>(Self::PATH_MARKET_PRICES)
            .await
            .map_err(Into::into)
    }

    /// TODO: Move
    #[instrument(err)]
    pub async fn fetch_regions(
        &self
    ) -> Result<Vec<RegionId>, MarketError> {
        self
            .client
            .fetch::<Vec<RegionId>>(Self::PATH_REGIONS)
            .await
            .map_err(Into::into)
    }
}

impl std::fmt::Debug for MarketService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProjectService").finish()
    }
}

/// Contains all errors that can happen in this service
#[derive(Debug)]
pub enum MarketError {
    /// Generic database error during execution
    DatabaseError(sqlx::Error),
    /// Error while parsing json
    SerdeError(serde_json::Error),
    /// Error from the eve client
    EveClientError(caph_connector::ConnectError),
}

crate::error_derive!(MarketError);

impl From<caph_connector::ConnectError> for MarketError {
    fn from(x: caph_connector::ConnectError) -> Self {
        Self::EveClientError(x)
    }
}

/// Represents a single market order
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketOrder {
    /// Determines if the order is buy or sell
    pub is_buy_order: bool,
    /// Price of the order
    pub price:        f64,
    /// System of the order
    pub system_id:    SystemId,
    /// [TypeId] of the sold item
    pub type_id:      TypeId,
}

/// Represents a single item and its market prices
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketPrice {
    /// Adjusted price of the item
    #[serde(default)]
    pub adjusted_price: f64,
    /// Average price of the item
    #[serde(default)]
    pub average_price:  f64,
    /// TypeID of the item
    pub type_id:        TypeId,
}

/// Prices of a single item
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketItemPrice {
    /// Minimum available price
    pub min:     f64,
    /// Maximum available price
    pub max:     f64,
    /// Average price, outliners ignored
    pub avg:     f64,
    /// TypeID of the item
    pub type_id: TypeId,
    /// Name of the item
    pub name:    String,
}
