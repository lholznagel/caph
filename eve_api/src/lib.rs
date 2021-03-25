mod character;
mod error;
mod market;
mod oauth;

pub use self::character::*;
pub use self::error::*;
pub use self::market::*;
pub use self::oauth::*;

use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;

/// This struct contains all functions for communicating with the Eve Online
/// REST API.
#[derive(Default)]
pub struct EveClient;

impl EveClient {
    const BASE_ADDR: &'static str = "https://esi.evetech.net/latest";

    /// Wraps reqwest´s client
    /// When requesting the eve online API often the server returns 502 or 503
    /// this results in a broken payload. If that happens, we just retry the request.
    /// The function will try 3 times, after that it will return an error.
    async fn fetch(&self, path: &str) -> Result<Response, EveApiError> {
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
            if response.status() != StatusCode::OK &&
               response.status() != StatusCode::NOT_FOUND {
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

    async fn fetch_oauth(
        &self,
        token: &str,
        path: &str
    ) -> Result<Response, EveApiError> {
        let mut retry_counter = 0;

        loop {
            let url = format!("{}/{}", EveClient::BASE_ADDR, path);
            if retry_counter == 3 {
                log::error!("Too many retries requesting {}.", url);
                return Err(EveApiError::TooManyRetries(url));
            }

            let response = reqwest::Client::new()
                .get(&url)
                .bearer_auth(token)
                .send()
                .await;
            let response = response.map_err(EveApiError::ReqwestError)?;

            if response.status() == StatusCode::UNAUTHORIZED ||
               response.status() == StatusCode::FORBIDDEN {
                return Err(EveApiError::Unauthorized);
            }

            // status 200 and 404 are ok
            if response.status() != StatusCode::OK &&
               response.status() != StatusCode::NOT_FOUND {
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

    async fn fetch_page<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<Vec<T>, EveApiError> {
        let response = self
            .fetch(path)
            .await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(Vec::new());
        }

        let pages = self.page_count(&response);

        let mut fetched_data: Vec<T> = Vec::new();
        fetched_data.extend(response.json::<Vec<T>>().await?);

        for page in 2..=pages {
            let next_page = self
                .fetch(&format!(
                    "{}?page={}",
                    path,
                    page
                ))
                .await?
                .json::<Vec<T>>()
                .await
                .map_err(EveApiError::ReqwestError)?;

            fetched_data.extend(next_page);
        }

        Ok(fetched_data)
    }

    async fn fetch_page_oauth<T: DeserializeOwned>(
        &self,
        token: &str,
        path: &str,
    ) -> Result<Vec<T>, EveApiError> {
        let response = self
            .fetch_oauth(token, path)
            .await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(Vec::new());
        }

        let pages = self.page_count(&response);

        let mut fetched_data: Vec<T> = Vec::new();
        fetched_data.extend(response.json::<Vec<T>>().await?);

        for page in 2..=pages {
            let next_page = self
                .fetch(&format!(
                    "{}?page={}",
                    path,
                    page
                ))
                .await?
                .json::<Vec<T>>()
                .await
                .map_err(EveApiError::ReqwestError)?;

            fetched_data.extend(next_page);
        }

        Ok(fetched_data)
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
