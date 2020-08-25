use eve::*;
use num_format::{Locale, ToFormattedString};
use prettytable::{cell, row, Cell, Row, Table};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loggify::LogBuilder::new()
        .add_exclude("hyper".into())
        .add_exclude("reqwest".into())
        .set_level(log::Level::Debug)
        .build()
        .unwrap();

    Database::create().await?;
    blueprint_item_market().await?;

    Ok(())
}

async fn blueprint_item_market() -> Result<(), EveError> {
    let blueprints = filter_type_ids_by_market(RegionId(10000016))
        .await?
        .into_iter()
        .filter(|x| x.name.contains("Blueprint"))
        .collect::<Vec<TypeData>>();

    let non_blueprints = filter_type_ids_by_market(RegionId(10000016))
        .await?
        .into_iter()
        .filter(|x| {
            for blueprint in &blueprints {
                if x.type_id.0 + 1 == blueprint.type_id.0 {
                    return true;
                }
            }

            false
        })
        .collect::<Vec<TypeData>>();

    let item_blueprint = BlueprintItem::new(RegionId(10000016)).collect(non_blueprints).await?;

    let mut table = Table::new();
    table.add_row(row!["Item name", "Price", "Buyers", "Blueprint cost", "Potential market"]);

    for item in item_blueprint {
        if item.item.buyers == 0 {
            continue;
        }

        let blueprint_price = if let Some(x) = &item.blueprint {
            x.price as u32
        } else {
            continue;
        };

        table.add_row(row![
            item.item.item_name,
            (item.item.order.price as u32).to_formatted_string(&Locale::de),
            item.item.buyers.to_formatted_string(&Locale::de),
            item.item.sellers.to_formatted_string(&Locale::de),
            blueprint_price.to_formatted_string(&Locale::de),
            (item.item.potential_market as u32).to_formatted_string(&Locale::de)
        ]);
    }

    table.printstd();
    Ok(())
}

async fn _blueprint_ress_calc() -> Result<(), EveError> {
    let database = Database::load().await?;
    let result = BlueprintResourceCalc::new(database).collect("Cormorant Blueprint".into(), 20).await?;

    let mut table = Table::new();

    let headers = result
        .clone()
        .into_iter()
        .map(|x| x.name)
        .map(|x| Cell::new(&x))
        .collect();
    table.add_row(Row::new(headers));

    let value = result
        .into_iter()
        .map(|x| x.count)
        .map(|x| Cell::new(&x.to_formatted_string(&Locale::de)))
        .collect();
    table.add_row(Row::new(value));

    table.printstd();
    Ok(())
}

