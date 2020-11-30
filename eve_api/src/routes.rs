use crate::eve_client::*;

use serde::{Deserialize, Serialize};

impl EveClient {
    pub async fn fetch_route(
        &self,
        origin: SystemId,
        destination: SystemId,
        flag: Option<RouteFlag>,
    ) -> crate::error::Result<Vec<SystemId>> {
        let flag = flag.unwrap_or_default().as_string();

        let mut response = self
            .fetch(&format!("route/{}/{}?flag={}", origin.0, destination.0, flag))
            .await?;

        if response.status() == 404 {
            return Ok(Vec::new());
        }

        Ok(response.body_json().await?)
    }
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
            Self::Insecure => "insecure"
        }.into()
    }
}

impl Default for RouteFlag {
    fn default() -> Self {
        Self::Shortest
    }
}