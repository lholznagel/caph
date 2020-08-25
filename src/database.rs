mod blueprint;

use self::blueprint::*;

use crate::error::*;
use crate::eve::*;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub struct Database {
    pub blueprints: Vec<Blueprint>,
    pub type_data: Vec<TypeData>,
}

impl Database {
    pub async fn load() -> Result<Self> {
        let mut type_data_file = File::open("./database/type_data.json").unwrap();

        let mut data = Vec::new();
        type_data_file.read_to_end(&mut data).unwrap();

        Ok(Self {
            blueprints: Blueprint::parse().unwrap(),
            type_data: serde_json::from_slice(&data).unwrap(),
        })
    }

    pub async fn create() -> Result<()> {
        if !Path::new("./database").exists() {
            std::fs::create_dir("./database").unwrap();
        }

        if !Path::new("./database/blueprints.yaml").exists() {
            log::error!("'database/blueprints.yaml' does not exist. Please download 'sde' from 'https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip' and extract the .zip. After that copy the blueprints.yaml from 'sde/fsd/blueprints.yaml' into the database folder.");
            std::process::exit(1);
        }

        if !Path::new("./database/type_data.json").exists() {
            File::create("./database/type_data.json").unwrap();
        }

        let mut type_data_file = File::open("./database/type_data.json").unwrap();

        let mut data = Vec::new();
        type_data_file.read_to_end(&mut data).unwrap();

        let eve = Eve::default();

        let type_ids = eve.fetch_type_ids().await?;
        let mut data: Vec<TypeData> = if data.is_empty() {
            Vec::new()
        } else {
            serde_json::from_slice(&data).unwrap()
        };

        let type_ids = type_ids.clone().into_iter().skip(data.len()).collect();
        let type_data = eve.fetch_types(type_ids).await?;
        data.extend(type_data);

        let parsed = serde_json::to_string(&data).unwrap();
        let mut type_data_file = File::create("./database/type_data.json").unwrap();
        type_data_file.write_all(parsed.as_bytes()).unwrap();

        Ok(())
    }
}
