use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    core:   Option<String>,
    folder: String,
    plans:  Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct EvemonPlan {
    #[serde(rename = "entry", default)]
    entries: Vec<EvemonEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EvemonEntry {
    #[serde(alias = "skillID")]
    skill_id: u32,
    level:    u8,
}

#[derive(Debug, Serialize)]
pub struct Output {
    name:     String,
    skills:   Vec<EvemonEntry>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = File::open("skillplans/skillplans.json")?;
    let config: Vec<ConfigFile> = serde_json::from_reader(config)?;

    let mut all = Vec::new();
    for c in config {
        if let Some(x) = c.core {
            let mut file = File::open(format!("skillplans/Core/{}", x))?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let plan: EvemonPlan = quick_xml::de::from_str(&content)?;

            all.push(Output {
                name:   x,
                skills: plan.entries
            });
        }

        for s in c.plans {
            let mut file = File::open(format!("skillplans/{}/{}", c.folder, s))?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let plan: EvemonPlan = quick_xml::de::from_str(&content)?;

            all.push(Output {
                name:   s,
                skills: plan.entries
            });
        }
    }

    let file = File::create("./skillplans/skillplan.out.json")?;
    serde_json::to_writer(file, &all)?;

    Ok(())
}
