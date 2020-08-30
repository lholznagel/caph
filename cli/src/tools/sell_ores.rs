use crate::database::Database;
use crate::error::*;
use crate::tools::SellItem;

use clap::Clap;
use num_format::{Locale, ToFormattedString};
use prettytable::{cell, row, Cell, Row, Table};
use std::sync::{Arc, Mutex};

pub struct SellOre {
    database: Arc<Mutex<Database>>,
}

impl SellOre {
    const ORES: &'static [&'static str] = &[
        "Veldspar",
        "Concentrated Veldspar",
        "Dense Veldspar",
        "Scordite",
        "Condensed Scordite",
        "Massive Scordite",
        "Pyroxeres",
        "Solid Pyroxeres",
        "Viscous Pyroxeres",
        "Plagioclase",
        "Azure Plagioclase",
        "Rich Plagioclase",
        "Omber",
        "Silvery Omber",
        "Golden Omber",
        "Kernite",
        "Luminous Kernite",
        "Fiery Kernite",
        "Jaspet",
        "Pure Jaspet",
        "Pristine Jaspet",
        "Hemorphite",
        "Vivid Hemorphite",
        "Radiant Hemorphite",
        "Hedbergite",
        "Vitric Hedbergite",
        "Glazed Hedbergite",
        "Gneiss",
        "Iridescent Gneiss",
        "Prismatic Gneiss",
        "Ochre Dark Ochre",
        "Onyx Ochre",
        "Obsidian Ochre",
        "Spodumain",
        "Bright Spodumain",
        "Gleaming Spodumain",
        "Crokite",
        "Sharp Crokite",
        "Crystalline Crokite",
        "Arkonor",
        "Crimson Arkonor",
        "Prime Arkonor",
        "Bistot",
        "Triclinic Bistot",
        "Monoclinic Bistot",
        "Mercoxit",
        "Magma Mercoxit",
        "Vitreous Mercoxit",
    ];

    pub fn new(database: Arc<Mutex<Database>>) -> Self {
        Self { database }
    }

    pub async fn collect_and_print(&self, include: Vec<String>, entries: usize) -> Result<()> {
        let mut table = Table::new();
        table.add_row(row![
            "Item",
            "Price",
            "Total orders",
            "Price / Density",
            "System",
            "Sec",
            "Region"
        ]);

        let mut ores = Vec::new();
        dbg!(&include);
        for x in include {
            if x.ends_with('*') {
                let ore = &x[..x.len() - 1];

                SellOre::ORES
                    .into_iter()
                    .filter(|x| x.contains(ore))
                    .for_each(|x| ores.push(x.to_string()));
            } else {
                ores.push(x);
            }
        }

        SellItem::new(self.database.clone())
            .collect(ores, entries, None)
            .await?
            .into_iter()
            .for_each(|x| {
                let mut entry_price = String::new();
                let mut entry_price_density = String::new();
                let mut entry_volume_remain = String::new();
                let mut entry_system = String::new();
                let mut entry_system_sec = String::new();
                let mut entry_region = String::new();

                for entry in x.entries {
                    entry_price.push_str(&entry.price.to_string());
                    entry_price.push_str("\n");

                    let price_density = (entry.price / x.density).round() as usize;
                    let price_density_str = price_density.to_string();
                    if price_density >= 275 {
                        entry_price_density.push_str("\x1B[0;34m");
                        entry_price_density
                            .push_str(&price_density_str);
                    } else if price_density >= 200 {
                        entry_price_density.push_str("\x1B[0;93m");
                        entry_price_density
                            .push_str(&price_density_str);
                    } else {
                        entry_price_density.push_str("\x1B[0;31m");
                        entry_price_density
                            .push_str(&price_density_str);
                    }

                    for _ in price_density_str.len()..16 {
                        entry_price_density.push_str(" ");
                    }
                    entry_price_density.push_str("\x1B\n");

                    entry_volume_remain
                        .push_str(&entry.volume_remain.to_formatted_string(&Locale::de));
                    entry_volume_remain.push_str("\n");

                    entry_system.push_str(&entry.system.to_string());
                    entry_system.push_str("\n");

                    entry_system_sec.push_str(&entry.system_sec.to_string());
                    entry_system_sec.push_str("\n");

                    entry_region.push_str(&entry.region.to_string());
                    entry_region.push_str("\n");
                }

                table.add_row(Row::new(vec![
                    Cell::new(&x.item),
                    Cell::new(&entry_price),
                    Cell::new(&entry_volume_remain),
                    Cell::new(&entry_price_density),
                    Cell::new(&entry_system),
                    Cell::new(&entry_system_sec),
                    Cell::new(&entry_region),
                ]));
            });

        table.print_tty(true);

        Ok(())
    }
}

#[derive(Clap)]
pub struct SellOreCli {
    #[clap(long, short, multiple = true)]
    pub include: Vec<String>,
}
