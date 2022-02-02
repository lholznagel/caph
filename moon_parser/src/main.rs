use std::collections::HashMap;

use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};
use appraisal::{Appraisal, Janice};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    Janice::validate()?;
    let janice = Janice::init()?;

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_path("inputs/in.csv")?;

    let mut res: Vec<MoonEntry> = rdr
        .deserialize::<CsvMoonEntry>()
        .map(|x| x.unwrap())
        .map(MoonEntry::from)
        .collect::<Vec<_>>();

    let mut wtr = WriterBuilder::new()
        .delimiter(b';')
        .from_path("output/out.csv")
        .unwrap();

    //let mut a = &mut res.iter_mut().find(|x| x.id == 6689).unwrap();
    //a.calc(&janice).await?;

    for a in res.iter_mut() {
        a.calc(&janice).await;
        wtr.serialize(a).unwrap();
    }

    wtr.flush().unwrap();

    Ok(())
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
struct CsvMoonEntry {
    #[serde(rename = "ID")]
    id:        u16,
    #[serde(rename = "Region")]
    region:    String,
    #[serde(rename = "System")]
    system:    String,
    #[serde(rename = "P")]
    planet:    u8,
    #[serde(rename = "M")]
    moon:      u8,
    #[serde(rename = "Mineral#1")]
    mineral_1: String,
    #[serde(rename = "Mineral#2")]
    mineral_2: String,
    #[serde(rename = "Mineral#3")]
    mineral_3: Option<String>,
    #[serde(rename = "Mineral#4")]
    mineral_4: Option<String>,
    #[serde(rename = "Total %")]
    total:     String,
    #[serde(rename = "Rent (Individuals)")]
    rent:      String,
    #[serde(rename = "Rent (Coporations)")]
    rent_corp: String,
    #[serde(rename = "Status")]
    status:    Option<String>
}


#[derive(Default, Debug, Serialize)]
struct MoonEntry {
    id:          u16,
    system:      String,
    planet:      u8,
    moon:        u8,
    #[serde(skip)]
    minerals:    HashMap<u32, u32>,
    rating:      String,
    rent:        u32,
    renatbility: Option<f32>,
    worth_cycle: Option<f32>,
    status:      Option<String>,
}

impl MoonEntry {
    pub async fn calc(
        &mut self,
        appraisal: &Janice
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.minerals
            .iter()
            .map(|(k, v)| format!("{} {}",type_id_to_name(*k), *v))
            .collect::<Vec<_>>();

        let a = appraisal.create(false, data).await?;
        self.worth_cycle = Some(a.split_price);
        self.renatbility = Some(self.worth_cycle.unwrap() / self.rent as f32);

        Ok(())
    }
}

const AMOUNT: f32 = (7f32 * 24f32 * 40_000f32) / 10f32;
impl From<CsvMoonEntry> for MoonEntry {
    fn from(x: CsvMoonEntry) -> Self {
        let mut minerals = HashMap::new();
        let mut rating = String::new();

        let mut mineral_split = x.mineral_1.split(' ');
        let mineral_name: String = mineral_split.next().unwrap().into();
        let mineral_quan: f32 = mineral_split.next().unwrap().replace('(', "").replace("%)", "").parse().unwrap();
        let mineral_quan = (AMOUNT * (mineral_quan / 100f32)) as u32;
        reprocess(name_to_type_id(&mineral_name), mineral_quan, &mut minerals);
        rating = format!("{}{} ({})", rating, &mineral_name, name_to_rating(&mineral_name));

        let mut mineral_split = x.mineral_2.split(' ');
        let mineral_name: String = mineral_split.next().unwrap().into();
        let mineral_quan: f32 = mineral_split.next().unwrap().replace('(', "").replace("%)", "").parse().unwrap();
        let mineral_quan = (AMOUNT * (mineral_quan / 100f32)) as u32;
        reprocess(name_to_type_id(&mineral_name), mineral_quan, &mut minerals);
        rating = format!("{}, {} ({})", rating, &mineral_name, name_to_rating(&mineral_name));

        if let Some(y) = x.mineral_3 {
            let mut mineral_split = y.split(' ');
            let mineral_name: String = mineral_split.next().unwrap().into();
            let mineral_quan: f32 = mineral_split.next().unwrap().replace('(', "").replace("%)", "").parse().unwrap();
            let mineral_quan = (AMOUNT * (mineral_quan / 100f32)) as u32;
            reprocess(name_to_type_id(&mineral_name), mineral_quan, &mut minerals);
            rating = format!("{}, {} ({})", rating, &mineral_name, name_to_rating(&mineral_name));
        }

        if let Some(x) = x.mineral_4 {
            let mut mineral_split = x.split(' ');
            let mineral_name: String = mineral_split.next().unwrap().into();
            let mineral_quan: f32 = mineral_split.next().unwrap().replace('(', "").replace("%)", "").parse().unwrap();
            let mineral_quan = (AMOUNT * (mineral_quan / 100f32)) as u32;
            reprocess(name_to_type_id(&mineral_name), mineral_quan, &mut minerals);
            rating = format!("{}, {} ({})", rating, &mineral_name, name_to_rating(&mineral_name));
        }

        let rent = x.rent.replace(",", "").parse().unwrap();

        Self {
            id:          x.id,
            system:      x.system,
            planet:      x.planet,
            moon:        x.moon,
            minerals:    minerals,
            rating:      rating,
            rent:        rent,
            status:      x.status,
            renatbility: Option::None,
            worth_cycle: Option::None
        }
    }
}

const REPROCESS_EFF: f64 = 0.876f64;
fn reprocess(type_id: u32, quantity: u32, materials: &mut HashMap<u32, u32>) {
    let quantity = ((((quantity as f32 / 100f32) / 10f32).floor()) * 100f32) as u32;

    let mut insert = |name: &str, quantity: u32| {
        materials
            .entry(name_to_type_id(name))
            .and_modify(|x: &mut u32| *x += quantity)
            .or_insert(quantity);
    };

    match type_id {
        45499 => {
            let a = ((quantity * 2)  as f64 * REPROCESS_EFF).floor() as u32;
            let b = ((quantity * 4) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Evaporite Deposits", a);
            insert("Platinum", b);
        },
        45501 => {
            let a = ((quantity * 2)  as f64 * REPROCESS_EFF).floor() as u32;
            let b = ((quantity * 4) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Hydrocarbons", a);
            insert("Chromium", b);
        },
        45498 => {
            let a = ((quantity * 2)  as f64 * REPROCESS_EFF).floor() as u32;
            let b = ((quantity * 4) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Atmospheric Gases", a);
            insert("Cadmium", b);
        },
        45494 => {
            let a = ((quantity * 4) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Cobalt", a);
        },
        45500 => {
            let a = ((quantity * 2)  as f64 * REPROCESS_EFF).floor() as u32;
            let b = ((quantity * 4) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Silicates", a);
            insert("Vanadium", b);
        },
        45496 => {
            let a = ((quantity * 4) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Titanium", a);
        },
        45490 => {
            let a = ((quantity * 800) as f64 * REPROCESS_EFF).floor() as u32;
            let b = ((quantity * 40) as f64 * REPROCESS_EFF).floor() as u32;
            let c = ((quantity * 7) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Pyerite", a);
            insert("Mexallon", b);
            insert("Atmospheric Gases", c);
        },
        45497 => {
            let a = ((quantity * 4) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Tungsten", a);
        },
        45495 => {
            let a = ((quantity * 4) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Scandium", a);
        },
        45492 => {
            let a = ((quantity * 600) as f64 * REPROCESS_EFF).floor() as u32;
            let b = ((quantity * 40) as f64 * REPROCESS_EFF).floor() as u32;
            let c = ((quantity * 7) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Pyerite", a);
            insert("Mexallon", b);
            insert("Hydrocarbons", c);
        },
        45491 => {
            let a = ((quantity * 400) as f64 * REPROCESS_EFF).floor() as u32;
            let b = ((quantity * 40)  as f64 * REPROCESS_EFF).floor() as u32;
            let c = ((quantity * 7) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Pyerite", a);
            insert("Mexallon", b);
            insert("Evaporite Deposits", c);
        },
        45493 => {
            let a = ((quantity * 200) as f64 * REPROCESS_EFF).floor() as u32;
            let b = ((quantity * 40)  as f64 * REPROCESS_EFF).floor() as u32;
            let c = ((quantity * 7) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Pyerite", a);
            insert("Mexallon", b);
            insert("Silicates", c);
        },
        45512 => {
            let a = ((quantity * 2) as f64 * REPROCESS_EFF).floor() as u32;
            let b = ((quantity * 2)  as f64 * REPROCESS_EFF).floor() as u32;
            let c = ((quantity * 1) as f64 * REPROCESS_EFF).floor() as u32;
            let d = ((quantity * 2) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Hydrocarbons", a);
            insert("Scandium", b);
            insert("Platinum", c);
            insert("Promethium", d);
        },
        45513 => {
            let a = ((quantity * 2) as f64 * REPROCESS_EFF).floor() as u32;
            let b = ((quantity * 2)  as f64 * REPROCESS_EFF).floor() as u32;
            let c = ((quantity * 1) as f64 * REPROCESS_EFF).floor() as u32;
            let d = ((quantity * 2) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Silicates", a);
            insert("Titanium", b);
            insert("Cadmium", c);
            insert("Thulium", d);
        },
        45504 => {
            let a = ((quantity * 2) as f64 * REPROCESS_EFF).floor() as u32;
            let b = ((quantity * 1)  as f64 * REPROCESS_EFF).floor() as u32;
            let c = ((quantity * 5) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Hydrocarbons", a);
            insert("Scandium", b);
            insert("Caesium", c);
        },
        45511 => {
            let a = ((quantity * 2) as f64 * REPROCESS_EFF).floor() as u32;
            let b = ((quantity * 2)  as f64 * REPROCESS_EFF).floor() as u32;
            let c = ((quantity * 1) as f64 * REPROCESS_EFF).floor() as u32;
            let d = ((quantity * 2) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Evaporite Deposits", a);
            insert("Tungsten", b);
            insert("Chromium", c);
            insert("Neodymium", d);
        },
        45510 => {
            let a = ((quantity * 2) as f64 * REPROCESS_EFF).floor() as u32;
            let b = ((quantity * 2)  as f64 * REPROCESS_EFF).floor() as u32;
            let c = ((quantity * 1) as f64 * REPROCESS_EFF).floor() as u32;
            let d = ((quantity * 2) as f64 * REPROCESS_EFF).floor() as u32;
            insert("Atmospheric Gases", a);
            insert("Cobalt", b);
            insert("Vanadium", c);
            insert("Dysprosium", d);
        },
        0 => {},
        _ => panic!("Invalid type_id {}", type_id)
    }
}

fn name_to_type_id(name: &str) -> u32 {
    match name {
        "Atmospheric Gases"  => 16634,
        "Bitumens"           => 45492,
        "Cadmium"            => 16643,
        "Caesium"            => 16647,
        "Chromite"           => 45501,
        "Chromium"           => 16641,
        "Cobalt"             => 16640,
        "Cobaltite"          => 45494,
        "Coesite"            => 45493,
        "Dysprosium"         => 16650,
        "Euxenite"           => 45495,
        "Evaporite Deposits" => 16635,
        "Hydrocarbons"       => 16633,
        "Loparite"           => 45512,
        "Mexallon"           => 36,
        "Monazite"           => 45511,
        "Neodymium"          => 16651,
        "Otavite"            => 45498,
        "Platinum"           => 16644,
        "Pollucite"          => 45504,
        "Promethium"         => 16652,
        "Pyerite"            => 35,
        "Scandium"           => 16639,
        "Scheelite"          => 45497,
        "Silicates"          => 16636,
        "Sperrylite"         => 45499,
        "Sylvite"            => 45491,
        "Thulium"            => 16653,
        "Titanite"           => 45496,
        "Titanium"           => 16638,
        "Tungsten"           => 16637,
        "Vanadinite"         => 45500,
        "Vanadium"           => 16642,
        "Xenotime"           => 45510,
        "Ytterbite"          => 45513,
        "Zeolites"           => 45490,

        _                    => panic!("Invalid name {}", name)
    }
}

fn name_to_rating(name: &str) -> &str {
    match name {
        "Monazite" | "Xenotime" | "Loparite" | "Ytterbite"   => "R64",
        "Zircon" | "Carnotite" | "Cinnbar" | "Pollucite"     => "R32",
        "Sperrylite" | "Otavite" | "Chromite" | "Vanadinite" => "R16",
        "Cobaltite" | "Titanite" | "Euxenite" | "Scheelite"  => "R8",
        "Zeolites" | "Bitumens" | "Sylvite" | "Coesite"      => "R4",
        _                                                    => panic!("Invalid name {}", name)
    }
}

fn type_id_to_name(type_id: u32) -> String {
    match type_id {
        16633 => "Hydrocarbons",
        16634 => "Atmospheric Gases",
        16635 => "Evaporite Deposits",
        16636 => "Silicates",
        16637 => "Tungsten",
        16638 => "Titanium",
        16639 => "Scandium",
        16640 => "Cobalt",
        16641 => "Chromium",
        16642 => "Vanadium",
        16643 => "Cadmium",
        16644 => "Platinum",
        16647 => "Caesium",
        16650 => "Dysprosium",
        16651 => "Neodymium",
        16652 => "Promethium",
        16653 => "Thulium",
        35    => "Pyerite",
        36    => "Mexallon",
        _     => panic!("Invalid type_id {}", type_id)
    }.into()
}
