use crate::*;

#[derive(Clone, Debug)]
pub struct MarketService {
    eve_client: EveClient,
}

impl MarketService {
    pub fn new(
        eve_client: EveClient,
        _: SdeZipArchive
    ) -> Result<Self, EveConnectError> {
        Ok(Self {
            eve_client
        })
    }

    /// Fetches all market orders for the given region id
    pub async fn orders<T: Into<RegionId>>(
        &self,
        rid: T,
    ) -> Result<Vec<MarketOrder>, EveConnectError> {
        self.eve_client
            .fetch_page(&format!("markets/{}/orders", *rid.into()))
            .await
    }

    /// Fetches historic values
    pub async fn history(
        &self,
        region_id: RegionId,
        type_id:   TypeId,
    ) -> Result<Vec<MarketHistory>, EveConnectError> {
        self.eve_client
            .fetch(&format!(
            "markets/{}/history?type_id={}",
            *region_id, *type_id
        ))
        .await?
        .json()
        .await
        .map_err(Into::into)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketOrder {
    pub duration:      u32,
    pub is_buy_order:  bool,
    pub issued:        String,
    pub location_id:   u64,
    pub min_volume:    u32,
    pub order_id:      u64,
    pub price:         f32,
    pub range:         String,
    pub system_id:     u32,
    pub type_id:       u32,
    pub volume_remain: u32,
    pub volume_total:  u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketHistory {
    pub average:     f32,
    pub highest:     f32,
    pub lowest:      f32,
    pub date:        String,
    pub order_count: u64,
    pub volume:      u64,
}
