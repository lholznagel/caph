use crate::database::Database;
use crate::error::*;

use num_format::{Locale, ToFormattedString};
use prettytable::{cell, row, Table};
use std::sync::{Arc, Mutex};

macro_rules! resource {
    ($name:ident, $($ore:expr => $amount:expr),*) => {
        fn $name(efficiency: f32) -> ReprocessingCalc {
            ReprocessingCalc {
                efficiency,
                resources: vec![
                    $(
                        RefinedResource { name: $ore.into(), amount: $amount },
                    )*
                ],
            }
        }
    };
}

pub struct Misc {
    database: Arc<Mutex<Database>>,
}

// 275683 Omber
// 2.204.800 / 871.337 / 1.333.463
// 275.600 / 108.918 / 166.682
// 234.260 / 92.500 / 141.680
// 544.348,06 ISK
// 345.366 Tritanium   {"adjusted_price":5.18,"average_price":7.37,"type_id":34}
//  38.670 Pyerite     {"adjusted_price":4.64,"average_price":5.88,"type_id":35}
// 160.310 Isogen      {"adjusted_price":22.63,"average_price":23.47,"type_id":37}
impl Misc {
    pub fn new(database: Arc<Mutex<Database>>) -> Self {
        Self { database }
    }

    pub async fn collect_and_print(&self) -> Result<()> {
        let efficiency = Efficiency {
            reprocessing: 4usize,
            efficiency: 4usize,
        };
        let ore_count = 100f32;
        let reprocessing = Self::omber(efficiency.get(0usize));

        let result = reprocessing.get(OreVariant::Tier1);

        let mut names = result
            .clone()
            .into_iter()
            .map(|x| x.name)
            .collect::<Vec<String>>();
        names.push("Omber".into());

        let item_pricing = crate::SellItem::new(self.database.clone())
            .collect(names.clone(), 1, Some(vec!["Sinq Laison".into()]))
            .await?;

        let _ = crate::SellItem::new(self.database.clone())
            .collect_raw(names, 1)
            .await?
            .into_iter()
            .for_each(|x| {
                dbg!(x.type_id);
            });

        let mut win_repro = 0f32;
        for x in result {
            let pricing = item_pricing
                .clone()
                .into_iter()
                .find(|y| y.item == x.name)
                .unwrap();

            win_repro += pricing.entries.get(0).unwrap().price * (x.amount * ore_count);
            dbg!(x.amount);
            dbg!(ore_count);
            //let c = ((x.amount * ore_count) as usize).to_formatted_string(&Locale::de);
        }

        let mut win_ore = 0f32;
        item_pricing
            .clone()
            .into_iter()
            .find(|y| y.item == String::from("Omber"))
            .map(|x| {
                dbg!(x.entries.get(0).unwrap().price);
                win_ore = x.entries.get(0).unwrap().price * ore_count;
            });

        let mut table = Table::new();
        table.add_row(row!["", "Win"]);

        table.add_row(row![
            "Ore",
            (win_ore as usize).to_formatted_string(&Locale::de)
        ]);
        table.add_row(row![
            "Reprocessing",
            (win_repro as usize).to_formatted_string(&Locale::de)
        ]);

        table.printstd();

        Ok(())
    }

    resource!(omber, "Tritanium" => 800f32, "Pyerite" => 100f32, "Isogen" => 85f32);
    resource!(veldspar, "Tritanium" => 415f32);
    resource!(scordite, "Tritanium" => 346f32, "Pyerite" => 173f32);
    resource!(pyroxeres, "Tritanium" => 351f32, "Pyerite" => 25f32, "Mexallon" => 50f32, "Nocxium" => 5f32);
    resource!(plagioclase, "Tritanium" => 107f32, "Pyerite" => 213f32, "Mexallon" => 107f32);
    resource!(kernite, "Tritanium" => 134f32, "Mexallon" => 267f32, "Isogen" => 134f32);
    resource!(jaspet, "Mexallon" => 350f32, "Nocxium" => 75f32, "Zydrine" => 8f32);
    resource!(hemorphite, "Tritanium" => 2200f32, "Isogen" => 100f32, "Nocxium" => 120f32, "Zydrine" => 15f32);
    resource!(hedbergite, "Pyerite" => 1000f32, "Isogen" => 200f32, "Nocxium" => 100f32, "Zydrine" => 19f32);
}

enum OreVariant {
    Tier1,
    Tier2,
    Tier3,
}

#[derive(Debug)]
struct ReprocessingCalc {
    efficiency: f32,
    resources: Vec<RefinedResource>,
}

impl ReprocessingCalc {
    pub fn new(efficiency: f32, resources: Vec<RefinedResource>) -> Self {
        Self {
            efficiency,
            resources,
        }
    }

    pub fn get(&self, tier: OreVariant) -> Vec<RefinedResource> {
        match tier {
            OreVariant::Tier1 => self.tier1(),
            OreVariant::Tier2 => self.tier2(),
            OreVariant::Tier3 => self.tier3(),
        }
    }

    fn tier1(&self) -> Vec<RefinedResource> {
        let mut result = Vec::with_capacity(self.resources.len());

        for ress in &self.resources {
            result.push(RefinedResource {
                name: ress.name.clone(),
                amount: ress.amount * self.efficiency,
            });
        }

        result
    }

    fn tier2(&self) -> Vec<RefinedResource> {
        let mut result = Vec::with_capacity(self.resources.len());

        for ress in &self.resources {
            result.push(RefinedResource {
                name: ress.name.clone(),
                amount: ((ress.amount + (ress.amount * 0.05)) * self.efficiency),
            });
        }

        result
    }

    fn tier3(&self) -> Vec<RefinedResource> {
        let mut result = Vec::with_capacity(self.resources.len());

        for ress in &self.resources {
            result.push(RefinedResource {
                name: ress.name.clone(),
                amount: ((ress.amount + (ress.amount * 0.1)) * self.efficiency),
            });
        }

        result
    }
}

#[derive(Clone, Debug)]
struct RefinedResource {
    name: String,
    amount: f32,
}

#[derive(Debug, Default)]
struct Efficiency {
    reprocessing: usize,
    efficiency: usize,
}

impl Efficiency {
    pub fn get(&self, ore_skill: usize) -> f32 {
        let r = match self.reprocessing {
            0 => 50.000,
            1 => 51.500,
            2 => 53.000,
            3 => 54.500,
            4 => match self.efficiency {
                0 => match ore_skill {
                    0 => 56.000,
                    1 => 57.120,
                    2 => 58.240,
                    3 => 59.360,
                    4 => 60.480,
                    5 => 61.600,
                    _ => panic!("Unknown ore skill"),
                },
                1 => match ore_skill {
                    0 => 57.120,
                    1 => 58.262,
                    2 => 59.405,
                    3 => 60.547,
                    4 => 61.690,
                    5 => 62.832,
                    _ => panic!("Unknown ore skill"),
                },
                2 => match ore_skill {
                    0 => 58.240,
                    1 => 59.405,
                    2 => 60.570,
                    3 => 61.734,
                    4 => 62.899,
                    5 => 64.064,
                    _ => panic!("Unknown ore skill"),
                },
                3 => match ore_skill {
                    0 => 59.360,
                    1 => 60.547,
                    2 => 61.734,
                    3 => 62.922,
                    4 => 64.109,
                    5 => 65.296,
                    _ => panic!("Unknown ore skill"),
                },
                4 => match ore_skill {
                    0 => 60.480,
                    1 => 61.690,
                    2 => 62.899,
                    3 => 64.109,
                    4 => 65.318,
                    5 => 66.528,
                    _ => panic!("Unknown ore skill"),
                },
                _ => panic!("Unknown efficiency skill"),
            },
            5 => match self.efficiency {
                4 => match ore_skill {
                    0 => 62.100,
                    1 => 63.342,
                    2 => 64.584,
                    3 => 65.826,
                    4 => 67.068,
                    5 => 68.310,
                    _ => panic!("Unknown ore skill"),
                },
                5 => match ore_skill {
                    0 => 63.250,
                    1 => 64.515,
                    2 => 65.780,
                    3 => 67.045,
                    4 => 68.310,
                    5 => 69.575,
                    _ => panic!("Unknown ore skill"),
                },
                _ => panic!("Unknown efficiency skill"),
            },
            _ => panic!("Unknown reporcessing skill"),
        };
        r / 100f32
    }
}
