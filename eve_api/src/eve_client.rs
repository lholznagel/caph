use crate::error::*;
use crate::id;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surf::{Client, Response};

pub trait Id {
    fn id(&self) -> u32;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default)]
pub struct EveClient;

id!(AllianceId, fetch_alliance_ids, "alliances/?page=");
id!(AttributeId, fetch_attribute_ids, "dogma/attributes/?page=");
id!(CategoryId, fetch_category_ids, "universe/categories/?page=");
id!(
    ConstellationId,
    fetch_constellation_ids,
    "universe/constellations/?page="
);
id!(EffectId, fetch_effect_ids, "dogma/effects/?page=");
id!(GraphicId, fetch_graphic_ids, "universe/graphics/?page=");
id!(GroupId, fetch_group_ids, "universe/groups/?page=");
id!(RegionId, fetch_region_ids, "universe/regions/?page=");
id!(
    StructureId,
    fetch_structure_ids,
    "universe/structures/?page="
);
id!(SystemId, fetch_system_ids, "universe/systems/?page=");
id!(TypeId, fetch_type_ids, "universe/types/?page=");

impl EveClient {
    pub(crate) const BASE_ADDR: &'static str = "https://esi.evetech.net/latest";
    /// Wraps reqwest´s client
    /// When requesting the eve online API often the server returns 502 or 503
    /// this results in a broken payload. If that happens, we just retry the request.
    /// The function will try 3 times, after that it will return an error.
    pub(crate) async fn fetch(&self, path: &str) -> Result<Response> {
        let mut retry_counter = 0;

        loop {
            let url = format!("{}/{}", EveClient::BASE_ADDR, path);

            let response = Client::new()
                .get(&url)
                .send()
                .await
                .map_err(EveApiError::ReqwestError)?;
            dbg!(response.status());

            // status 200 and 404 are ok
            if response.status() != 200 && response.status() != 404 {
                retry_counter += 1;
                log::error!(
                    "Fetch resulted in non 200 or 404 status code. Statuscode was {}. Retrying.",
                    response.status()
                );
                continue;
            }

            if retry_counter == 3 {
                log::error!("Too many retries requesting {}.", url);
                return Err(EveApiError::TooManyRetries(url));
            }

            return Ok(response);
        }
    }

    // path = universe/systems/?datasource=tranquility&page=
    pub(crate) async fn fetch_ids<T: DeserializeOwned>(&self, path: &str) -> Result<Vec<T>> {
        let mut response = self.fetch(&format!("{}{}", path, 1)).await?;

        let pages = self.page_count(&response);
        log::debug!("[EVE_API] Downloaded page  1 from {}", pages);

        let mut ids: Vec<T> = Vec::new();
        ids.extend(response.body_json::<Vec<T>>().await?);

        for page in 2..=pages {
            let next_page = self
                .fetch(&format!("{}{}", path, page))
                .await?
                .body_json::<Vec<T>>()
                .await
                .map_err(EveApiError::ReqwestError)?;

            ids.extend(next_page);
            log::debug!("[EVE_API] Downloaded page {:2} from {}", page, pages);
        }

        log::debug!("[EVE_API] Downloaded {} ids", ids.len());
        Ok(ids)
    }

    pub(crate) async fn fetch_by_id<T: DeserializeOwned>(
        &self,
        path: &str,
        id: u32,
        sub_path: Option<&str>,
    ) -> Result<Option<Vec<T>>> {
        let mut response = self
            .fetch(&format!("{}/{}/{}", path, id, sub_path.unwrap_or_default()))
            .await?;

        if response.status() == 404 {
            return Ok(None);
        }

        let pages = self.page_count(&response);

        let mut fetched_data: Vec<T> = Vec::new();
        fetched_data.extend(response.body_json::<Vec<T>>().await?);

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
                .body_json::<Vec<T>>()
                .await
                .map_err(EveApiError::ReqwestError)?;

            fetched_data.extend(next_page);
        }

        Ok(Some(fetched_data))
    }

    pub(crate) async fn fetch_single_by_id<T: DeserializeOwned>(
        &self,
        path: &str,
        id: u32,
        sub_path: Option<&str>,
    ) -> Result<Option<T>> {
        let mut response = self
            .fetch(&format!("{}/{}/{}", path, id, sub_path.unwrap_or_default()))
            .await?;

        if response.status() == 404 {
            return Ok(None);
        }

        Ok(Some(response.body_json().await?))
    }

    pub(crate) fn page_count(&self, response: &Response) -> u8 {
        if let Some(x) = response.header("x-pages") {
            x.as_str().parse::<u8>().unwrap_or_default()
        } else {
            0u8
        }
    }
}
