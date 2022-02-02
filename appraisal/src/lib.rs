//! Wrapper for different praisal sites.
//! 
//! Currently supports [janice](https://janice.e-351.com/) and [evepraisal](https://evepraisal.com/).

#![forbid(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_Error_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
)]
#![warn(
    clippy::await_holding_lock,
    clippy::get_unwrap,
    clippy::map_unwrap_or,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
)]
#![allow(
    clippy::redundant_field_names
)]

/// Contains all errors that can happen during runtime
pub mod error;
pub use self::error::Error;

/// Apprisal implementation for [janice](https://janice.e-351.com/)
pub mod janice;
pub use self::janice::Janice;

use async_trait::*;
use serde::Serialize;

/// Generalized trait for communicating with a praisal site.
#[async_trait]
pub trait Appraisal {
    /// Validates that all required Environment variables are set
    fn validate() -> Result<(), Error>;

    /// Creates a new appraisal instance
    fn init() -> Result<Self, Error> where Self: Sized;

    /// Creates a new appraisal
    async fn create(
        &self,
        persist: bool,
        entries: Vec<String>
    ) -> Result<AppraisalInformation, Error>;
}

/// Generalized Appraisal information
#[derive(Debug, Serialize)]
pub struct AppraisalInformation {
    /// Sell price for all items
    pub sell_price:  f32,
    /// Split price for all items
    pub split_price: f32,
    /// Buy price for all items
    pub buy_price:   f32,

    /// Breakdown of all items
    pub items:       Vec<AppraisalItem>,

    /// Optional code to share the appraisal
    pub code:        Option<String>,
    /// Uri for sharing the appraisal
    pub uri:         Option<String>
}

/// Single item for an appraisal
#[derive(Debug, Serialize)]
pub struct AppraisalItem {
    /// TypeId of the item
    pub type_id: u32,
    /// Name of the item
    pub name:    String,
    /// Amount that is required
    pub amount:  u64,

    /// Sell price for a single item
    pub sell_price:        f32,
    /// Split price for a single item
    pub split_price:       f32,
    /// Buy price for a single item
    pub buy_price:         f32,

    /// Total sell price for the required amount
    pub sell_price_total:  f32,
    /// Total split price for the required amount
    pub split_price_total: f32,
    /// Total buy price for the required amount
    pub buy_price_total:   f32,
}
