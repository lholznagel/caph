use caph_eve_data_wrapper::{EveDataWrapper, TypeId};
use std::collections::HashMap;

const THE_FORGE: u32 = 10000002;
const ROLLING_AVERAGE: u8 = 14;

// const MOON_ORE: u32 = 45511; // Monazite
// const QUANTITY: u32 = 6396;

// const MOON_ORE: u32 = 45500; // Vanadinite
// const QUANTITY: u32 = 3034;

// const MOON_ORE: u32 = 46300; // Lavish Vanadinite
// const QUANTITY: u32 = 7041;

const MOON_ORE: u32 = 45510;
const QUANTITY: u32 = 6853;

#[derive(Debug, Hash, PartialEq, Eq)]
enum Rarity {
    R4,
    R8,
    R16,
    R32,
    R64
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let eve_client = EveDataWrapper::new().await?;
    let market_service = eve_client.market().await?;
    let type_service = eve_client.types().await?;

    let mut rarity = HashMap::new();
    rarity.insert(TypeId(16633), Rarity::R4);
    rarity.insert(TypeId(16634), Rarity::R4);
    rarity.insert(TypeId(16635), Rarity::R4);
    rarity.insert(TypeId(16636), Rarity::R4);

    rarity.insert(TypeId(16637), Rarity::R8);
    rarity.insert(TypeId(16638), Rarity::R8);
    rarity.insert(TypeId(16639), Rarity::R8);
    rarity.insert(TypeId(16640), Rarity::R8);

    rarity.insert(TypeId(16641), Rarity::R16);
    rarity.insert(TypeId(16642), Rarity::R16);
    rarity.insert(TypeId(16643), Rarity::R16);
    rarity.insert(TypeId(16644), Rarity::R16);

    rarity.insert(TypeId(16646), Rarity::R32);
    rarity.insert(TypeId(16647), Rarity::R32);
    rarity.insert(TypeId(16648), Rarity::R32);
    rarity.insert(TypeId(16649), Rarity::R32);

    rarity.insert(TypeId(16650), Rarity::R64);
    rarity.insert(TypeId(16651), Rarity::R64);
    rarity.insert(TypeId(16652), Rarity::R64);
    rarity.insert(TypeId(16653), Rarity::R64);

    let mut taxes = HashMap::new();
    taxes.insert(Rarity::R4, 0u32);
    taxes.insert(Rarity::R8, 5u32);
    taxes.insert(Rarity::R16, 10u32);
    taxes.insert(Rarity::R32, 15u32);
    taxes.insert(Rarity::R64, 20u32);

    let reprocess = type_service.materials();

    let mut market_historic_vals = HashMap::new();

    for material in reprocess.get(&MOON_ORE.into()).unwrap().materials.clone() {
        let mut historic = market_service
            .history(THE_FORGE.into(), material.material_type_id)
            .await?;
        historic.reverse();

        let mut weight = ROLLING_AVERAGE;
        let mut weighted_avg = 0f32;
        let mut weighted_total = 0;
        for val in historic {
            if weight > 0 {
                weighted_avg += val.average * weight as f32;
                weighted_total += weight;
                weight -= 1;
            } else {
                break;
            }
        }

        let average_price = weighted_avg / weighted_total as f32;
        market_historic_vals.insert(material.material_type_id, average_price);
    }

    let rep_amount = QUANTITY as f32 / 100f32;
    let mut tax_amount = 0f32;
    for material in reprocess.get(&MOON_ORE.into()).unwrap().materials.clone() {
        let avg = market_historic_vals.get(&material.material_type_id).unwrap();
        let avg = f32::round(*avg);
        // Amount of goo mined * Moving average * quantity of refined
        let rep_result = rep_amount * (avg * (material.quantity as f32 * 0.86));

        // R4, R8, R16, R32, R64
        let rarity = rarity.get(&material.material_type_id).unwrap();
        let tax = *taxes.get(&rarity).unwrap() as f32 / 100f32;
        tax_amount += rep_result * tax;
    }

    dbg!(tax_amount);

    Ok(())
}
