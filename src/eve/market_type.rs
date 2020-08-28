use crate::error::*;
use crate::eve::*;

impl Eve {
    pub async fn fetch_region_market_types(&self, region_id: RegionId) -> Result<Vec<TypeId>> {
        let response = self
            .fetch(&format!(
                "markets/{}/types/?datasource=tranquility&page=1",
                region_id.0
            ))
            .await?;

        let pages = self.page_count(&response);
        log::debug!("Downloaded Market TypeIds page  1 from {}", pages);

        let mut type_ids: Vec<TypeId> = Vec::with_capacity((pages as u16 * 1_000) as usize);

        for page in 2..=pages {
            let next_page = self
                .fetch(&format!(
                    "markets/{}/types/?datasource=tranquility&page={}",
                    region_id.0, page
                ))
                .await?
                .json::<Vec<TypeId>>()
                .await
                .map_err(EveError::ReqwestError)?;

            type_ids.extend(next_page);
            log::debug!("Downloaded Market TypeIds page {:2} from {}", page, pages);
        }

        let first_request_type_ids: Vec<TypeId> = response.json().await?;
        type_ids.extend(first_request_type_ids);

        log::debug!("Downloaded {} Market TypeIds", type_ids.len());
        Ok(type_ids)
    }
}
