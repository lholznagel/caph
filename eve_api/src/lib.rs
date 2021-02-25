mod error;
mod oauth;

use self::error::*;
pub use self::oauth::*;

use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Default)]
pub struct EveClient;

impl EveClient {
    const BASE_ADDR: &'static str = "https://esi.evetech.net/latest";

    /// Fetches all market orders for the given region id
    pub async fn fetch_market_orders(&self, region_id: u32) -> Result<Option<Vec<MarketOrder>>> {
        self.fetch_by_id("markets", region_id, Some("orders")).await
    }

    pub async fn fetch_market_history(
        &self,
        region_id: u32,
        type_id: u32,
    ) -> Result<Vec<MarketHistory>> {
        self.fetch(&format!(
            "markets/{}/history?type_id={}",
            region_id, type_id
        ))
        .await?
        .json::<Vec<MarketHistory>>()
        .await
        .map_err(Into::into)
    }

    pub async fn fetch_route(
        &self,
        origin: u32,
        destination: u32,
        flag: Option<RouteFlag>,
    ) -> Result<Vec<u32>> {
        let flag = flag.unwrap_or_default().as_string();

        let response = self
            .fetch(&format!("route/{}/{}?flag={}", origin, destination, flag))
            .await?;

        if response.status() == 404 {
            return Ok(Vec::new());
        }

        Ok(response.json().await?)
    }

    /// Wraps reqwest´s client
    /// When requesting the eve online API often the server returns 502 or 503
    /// this results in a broken payload. If that happens, we just retry the request.
    /// The function will try 3 times, after that it will return an error.
    async fn fetch(&self, path: &str) -> Result<Response> {
        let mut retry_counter = 0;

        loop {
            let url = format!("{}/{}", EveClient::BASE_ADDR, path);
            if retry_counter == 3 {
                log::error!("Too many retries requesting {}.", url);
                return Err(EveApiError::TooManyRetries(url));
            }

            let response = reqwest::get(&url).await;
            let response = response.map_err(EveApiError::ReqwestError)?;

            // status 200 and 404 are ok
            if response.status() != 200 && response.status() != 404 {
                retry_counter += 1;
                log::error!(
                    "Fetch resulted in non 200 or 404 status code. Statuscode was {}. Retrying.",
                    response.status()
                );
                continue;
            }

            return Ok(response);
        }
    }

    async fn fetch_by_id<T: DeserializeOwned>(
        &self,
        path: &str,
        id: u32,
        sub_path: Option<&str>,
    ) -> Result<Option<Vec<T>>> {
        let response = self
            .fetch(&format!("{}/{}/{}", path, id, sub_path.unwrap_or_default()))
            .await?;

        if response.status() == 404 {
            return Ok(None);
        }

        let pages = self.page_count(&response);

        let mut fetched_data: Vec<T> = Vec::new();
        fetched_data.extend(response.json::<Vec<T>>().await?);

        for page in 2..=pages {
            let next_page = self
                .fetch(&format!(
                    "{}/{}/{}?page={}",
                    path,
                    id,
                    sub_path.unwrap_or_default(),
                    page
                ))
                .await?
                .json::<Vec<T>>()
                .await
                .map_err(EveApiError::ReqwestError)?;

            fetched_data.extend(next_page);
        }

        Ok(Some(fetched_data))
    }

    fn page_count(&self, response: &Response) -> u8 {
        let headers = response.headers();
        if let Some(x) = headers.get("x-pages") {
            x.to_str()
                .unwrap_or_default()
                .parse::<u8>()
                .unwrap_or_default()
        } else {
            0u8
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketOrder {
    pub duration: u32,
    pub is_buy_order: bool,
    pub issued: String,
    pub location_id: u64,
    pub min_volume: u32,
    pub order_id: u64,
    pub price: f32,
    pub range: String,
    pub system_id: u32,
    pub type_id: u32,
    pub volume_remain: u32,
    pub volume_total: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketHistory {
    pub average: f32,
    pub highest: f32,
    pub lowest: f32,
    pub date: String,
    pub order_count: u64,
    pub volume: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RouteFlag {
    Shortest,
    Secure,
    Insecure,
}

impl RouteFlag {
    pub fn as_string(self) -> String {
        match self {
            Self::Shortest => "shortest",
            Self::Secure => "secure",
            Self::Insecure => "insecure",
        }
        .into()
    }
}

impl Default for RouteFlag {
    fn default() -> Self {
        Self::Shortest
    }
}
