mod buy;
mod misc;
mod sell;
mod sell_ores;

pub use self::buy::*;
pub use self::misc::*;
pub use self::sell::*;
pub use self::sell_ores::*;

use crate::error::*;
use crate::Database;

use caph_eve_online_api::{RegionId, TypeId};
use std::sync::{Arc, Mutex};

pub(crate) async fn resolve_items(
    database: Arc<Mutex<Database>>,
    item_names: Vec<String>,
) -> Result<Vec<TypeId>> {
    let progress_bar = crate::new_progress_bar();
    progress_bar.set_message("Resolving item names");

    let items = item_names
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut database = database.lock().unwrap();
    let ids = database.resolve_item_ids(items).await?;
    progress_bar.finish_with_message("Resolved item names");

    Ok(ids)
}

pub(crate) async fn resolve_regions(
    database: Arc<Mutex<Database>>,
    regions: Option<Vec<String>>,
) -> Result<Vec<RegionId>> {
    let progress = crate::new_progress_bar();
    progress.set_message("Resolving RegionIds");

    let mut db = database.lock().unwrap();
    let regions = regions.unwrap_or(
        crate::DEFAULT_REGIONS
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    );

    let regions = db.resolve_region_ids(regions).await;

    progress.finish_with_message("Resolved RegionIds");
    regions
}
