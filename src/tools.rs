mod best_price;
mod blueprint;
mod blueprint_ress_calc;

pub use self::best_price::*;
pub use self::blueprint::*;
pub use self::blueprint_ress_calc::*;

use crate::database::*;
use crate::error::*;
use crate::eve::*;

pub async fn filter_type_ids_by_market(region: RegionId) -> Result<Vec<TypeData>> {
    let market_types = Eve::default()
        .fetch_region_market_types(region)
        .await?;
    
    let type_data = Database::load()
        .await?
        .type_data
        .into_iter()
        .filter(|x| {
            for market_type in &market_types {
                if x.type_id == *market_type {
                    return true;
                }
            }

            false
        })
        .collect();
    Ok(type_data)
}

pub async fn filter_type_ids_by_name(name: Vec<String>) -> Result<Vec<TypeData>> {
    let type_data = Database::load()
        .await?
        .type_data
        .into_iter()
        .filter(|x| {
            for filter in &name {
                if x.name == *filter {
                    return true;
                }
            }

            false
        })
        .collect::<Vec<TypeData>>();
    Ok(type_data)
}