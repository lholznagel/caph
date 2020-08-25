mod group_data;
mod market_order;
mod market_price;
mod market_type;
mod region_data;
mod system_data;
mod type_data;

pub use self::group_data::*;
pub use self::market_order::*;
pub use self::market_price::*;
pub use self::market_type::*;
pub use self::region_data::*;
pub use self::system_data::*;
pub use self::type_data::*;

use crate::error::*;

use reqwest::{Client, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Default)]
pub struct Eve;

impl Eve {
    const BASE_ADDR: &'static str = "https://esi.evetech.net/latest";

    async fn fetch(&self, path: &str) -> Result<Response> {
        loop {
            let response = Client::new()
                .get(&format!("{}/{}", Eve::BASE_ADDR, path))
                .send()
                .await
                .map_err(EveError::ReqwestError)?;

            // status 200 and 404 are ok
            if response.status() != 200 && response.status() != 404 {
                log::error!("Fetch resulted in non 200 status code. {}. Retrying.", response.status());
                continue;
            }

            return Ok(response);
        }
    }

    // path = universe/systems/?datasource=tranquility&page=
    async fn fetch_ids<T: DeserializeOwned>(&self, path: &str) -> Result<Vec<T>> {
        let response = self.fetch(&format!("{}{}", path, 1)).await?;

        let pages = self.page_count(&response);
        log::debug!("Downloaded page  1 from {}", pages);

        let mut ids: Vec<T> = Vec::with_capacity((pages as u16 * 1_000) as usize);
        ids.extend(response.json::<Vec<T>>().await?);

        for page in 2..=pages {
            let next_page = self
                .fetch(&format!("{}{}", path, page))
                .await?
                .json::<Vec<T>>()
                .await
                .map_err(EveError::ReqwestError)?;

            ids.extend(next_page);
            log::debug!("Downloaded page {:2} from {}", page, pages);
        }

        log::debug!("Downloaded {} ids", ids.len());
        Ok(ids)
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
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
