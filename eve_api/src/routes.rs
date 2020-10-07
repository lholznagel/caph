use crate::eve_client::*;

impl EveClient {
    pub async fn fetch_route(
        &self,
        origin: SystemId,
        destination: SystemId,
    ) -> crate::error::Result<Vec<SystemId>> {
        let mut response = self
            .fetch(&format!("route/{}/{}", origin.0, destination.0))
            .await?;

        if response.status() == 404 {
            return Ok(Vec::new());
        }

        Ok(response.body_json().await?)
    }
}
