use crate::EveClient;
use crate::error::*;

use serde::{Deserialize, Serialize};

impl EveClient {
    pub async fn portrait(
        &self,
        token: &str,
        character_id: u32,
    ) -> Result<String, EveApiError> {
        #[derive(Deserialize)]
        struct Portrait {
            #[serde(rename = "px512x512")]
            img: String,
        }

        let path = format!("characters/{}/portrait", character_id);
        self
            .fetch_oauth(&token, &path)
            .await?
            .json::<Portrait>()
            .await
            .map(|x| x.img)
            .map_err(Into::into)
    }

    pub async fn whoami(
        &self,
        token: &str,
        character_id: u32,
    ) -> Result<String, EveApiError> {
        #[derive(Deserialize)]
        struct Character {
            name: String,
        }

        let path = format!("characters/{}/", character_id);
        self
            .fetch_oauth(&token, &path)
            .await?
            .json::<Character>()
            .await
            .map(|x| x.name)
            .map_err(Into::into)
    }

    pub async fn assets(
        &self,
        token: &str,
        character_id: u32,
    ) -> Result<Vec<CharacterAsset>, EveApiError> {
        let path = format!("characters/{}/assets", character_id);
        self
            .fetch_page_oauth::<CharacterAsset>(&token, &path)
            .await
            .map_err(Into::into)
    }

    pub async fn blueprints(
        &self,
        token: &str,
        character_id: u32,
    ) -> Result<Vec<CharacterBlueprint>, EveApiError> {
        let path = format!("characters/{}/blueprints", character_id);
        self
            .fetch_page_oauth::<CharacterBlueprint>(&token, &path)
            .await
            .map_err(Into::into)
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterAsset {
    pub is_singleton: bool,
    pub item_id: u64,
    pub location_flag: String,
    pub location_id: u64,
    pub location_type: String,
    pub quantity: u32,
    pub type_id: u32,

    pub is_blueprint_copy: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterBlueprint {
    pub item_id: u64,
    pub location_flag: String,
    pub location_id: u64,
    pub material_efficiency: u32,
    /// A range of numbers with a minimum of -2 and no maximum value where -1
    /// is an original and -2 is a copy. It can be a positive integer if it is
    /// a stack of blueprint originals fresh from the market (e.g. no 
    /// activities performed on them yet).
    pub quantity: i32,
    /// Number of runs remaining if the blueprint is a copy, -1 if it is an original
    pub runs: i32,
    pub time_efficiency: u32,
    pub type_id: u32,
}
