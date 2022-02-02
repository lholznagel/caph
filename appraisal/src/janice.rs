use async_trait::*;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use std::collections::HashMap;

use crate::{Appraisal, AppraisalInformation, AppraisalItem, Error};

/// Implementation for [janice](https://janice.e-351.com/)
/// 
/// Additional documentation [swagger](https://janice.e-351.com/api/rest/docs/index.html)
pub struct Janice(reqwest::Client);

impl Janice {
    /// Name of the UserAgent ENV
    const USER_AGENT:    &'static str = "JANICE_USER_AGENT";
    /// Name of the ApiKey ENV
    const API_KEY:       &'static str = "JANICE_API_KEY";
    /// Url for creating appraisals
    const APPRAISAL_URL: &'static str = "https://janice.e-351.com/api/rest/v2/appraisal";
}

#[async_trait]
impl Appraisal for Janice where Self: Sized {
    /// Validates that all required Environment variables are set
    /// 
    /// # Error
    /// 
    /// Fails when a Environment-Variable is missing
    /// 
    /// # Returns
    /// 
    /// `Ok`  -> If all Environment-Variables are set
    /// `Err` -> Not all Environment-Variables are set, contains the missing ENV-Name
    /// 
    fn validate() -> Result<(), Error> {
        std::env::var(Self::USER_AGENT)
            .map_err(|_| Error::MissingEnv(format!("JANICE_{}", Self::USER_AGENT)))
            .map(drop)?;
        std::env::var(Self::API_KEY)
            .map_err(|_| Error::MissingEnv(format!("JANICE_{}", Self::API_KEY)))
            .map(drop)?;
        Ok(())
    }

    /// Creates a new janice appraisal instance.
    /// 
    /// # Error
    /// 
    /// If not all required Environment-Variables are set.
    /// 
    /// # Returns
    /// 
    /// Appraisal instance
    /// 
    fn init() -> Result<Self, Error> {
        let user_agent = std::env::var(Self::USER_AGENT)
            .map_err(|_| Error::MissingEnv(format!("JANICE_{}", Self::USER_AGENT)))?;
        let api_key = std::env::var(Self::API_KEY)
            .map_err(|_| Error::MissingEnv(format!("JANICE_{}", Self::API_KEY)))?;

        let mut headers = HeaderMap::new();
        headers.insert(
            "X-ApiKey",
            HeaderValue::from_str(&api_key).unwrap()
        );
        headers.insert(
            "Content-Type",
            HeaderValue::from_static("text/plain")
        );

        let client = Client::builder()
            .user_agent(user_agent)
            .default_headers(headers)
            .build()
            .map_err(Error::CouldNotConstructClient)?;

        Ok(Self(client))
    }

    /// Creates a new apprisal
    /// 
    /// # Params
    /// 
    /// * `persist` -> Determines if the apprisal should be stored
    /// * `entries` -> List of entries to create a apprisal for
    ///                Format: `item_name quantity`
    /// 
    /// # Errors
    /// 
    /// - When the server is not reachable
    /// - Invalid Format
    /// 
    /// # Returns
    /// 
    /// Appraisal information
    /// 
    async fn create(
        &self,
        persist: bool,
        entries: Vec<String>
    ) -> Result<AppraisalInformation, Error> {
        let mut params = HashMap::new();
        params.insert("persist", persist.to_string());
        params.insert("designation", "appraisal".into());
        params.insert("pricing", "split".into());
        params.insert("pricingVariant", "immediate".into());

        self.0
            .post(Self::APPRAISAL_URL)
            .query(&params)
            .body(entries.join("\n"))
            .send()
            .await
            .map_err(Error::RequestError)?
            .json::<ApprisalResponse>()
            .await
            .map_err(Error::RequestError)
            .map(Into::into)
    }
}

/// Represents the response from janice
/// 
/// Not all fields are represented
#[derive(Debug, Deserialize)]
pub struct ApprisalResponse {
    /// Effective price
    #[serde(rename = "effectivePrices")]
    pub effective_price: AppraisalValue,
    /// Price if sold immidiate
    #[serde(rename = "immediatePrices")]
    pub immidiate_price: AppraisalValue,
    /// Average price of the top 5
    #[serde(rename = "top5AveragePrices")]
    pub average_price:   AppraisalValue,

    /// Breakdown of all items
    pub items:           Vec<Item>,

    /// Optional code to share the appraisal
    pub code:            Option<String>,
}

impl Into<AppraisalInformation> for ApprisalResponse {
    fn into(self) -> AppraisalInformation {
        let uri = self.code
            .as_ref()
            .map(|x| format!("https://janice.e-351.com/a/{}", x));

        AppraisalInformation {
            sell_price:  self.average_price.sell_price,
            split_price: self.average_price.split_price,
            buy_price:   self.average_price.buy_price,
            items:       self.items.into_iter().map(Into::into).collect::<Vec<_>>(),
            code:        self.code,
            uri:         uri,
        }
    }
}

/// Represents an janice Appraisal Value
#[derive(Debug, Deserialize)]
pub struct Item {
    /// Effective price
    #[serde(rename = "effectivePrices")]
    pub effective_price: AppraisalValueItem,
    /// Price if sold immidiate
    #[serde(rename = "immediatePrices")]
    pub immidiate_price: AppraisalValueItem,
    /// Average price of the top 5
    #[serde(rename = "top5AveragePrices")]
    pub average_price:   AppraisalValueItem,

    /// Given amount of the item
    pub amount: u64,

    /// Information about the item
    #[serde(rename = "itemType")]
    pub item_type: ItemType,
}

impl Into<AppraisalItem> for Item {
    fn into(self) -> AppraisalItem {
        AppraisalItem {
            type_id: self.item_type.eid,
            name:    self.item_type.name,

            amount:  self.amount,

            sell_price:  self.average_price.sell_price,
            split_price: self.average_price.split_price,
            buy_price:   self.average_price.buy_price,

            sell_price_total:  self.average_price.sell_price_total,
            split_price_total: self.average_price.split_price_total,
            buy_price_total:   self.average_price.buy_price_total,
        }
    }
}

/// Represents an janice Appraisal Value
#[derive(Debug, Deserialize)]
pub struct ItemType {
    /// TypeId of the item
    pub eid:    u32,
    /// Name of the item
    pub name:   String,
}

/// Represents an janice Appraisal Value
#[derive(Debug, Deserialize)]
pub struct AppraisalValue {
    /// Buy price for all items
    #[serde(rename = "totalBuyPrice")]
    pub buy_price:   f32,
    /// Split price for all items
    #[serde(rename = "totalSplitPrice")]
    pub split_price: f32,
    /// Sell price for all items
    #[serde(rename = "totalSellPrice")]
    pub sell_price:  f32
}

/// Represents an janice Appraisal Item Value
#[derive(Debug, Deserialize)]
pub struct AppraisalValueItem {
    /// Buy price for one items
    #[serde(rename = "buyPrice")]
    pub buy_price:         f32,
    /// Split price for one items
    #[serde(rename = "splitPrice")]
    pub split_price:       f32,
    /// Sell price for one items
    #[serde(rename = "sellPrice")]
    pub sell_price:        f32,

    /// Buy price for all items
    #[serde(rename = "buyPriceTotal")]
    pub buy_price_total:   f32,
    /// Split price for all items
    #[serde(rename = "splitPriceTotal")]
    pub split_price_total: f32,
    /// Sell price for all items
    #[serde(rename = "sellPriceTotal")]
    pub sell_price_total:  f32,
}
