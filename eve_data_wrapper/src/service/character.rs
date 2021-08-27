use crate::*;

#[derive(Clone, Debug)]
pub struct CharacterService {
    eve_client: EveClient,
}

impl CharacterService {
    pub(crate) fn new(
        eve_client: EveClient,
        _: SdeZipArchive,
    ) -> Result<Self, EveConnectError> {
        Ok(Self {
            eve_client
        })
    }

    pub async fn portrait(
        &self,
        token: &str,
        character_id: u32,
    ) -> Result<String, EveConnectError> {
        #[derive(Deserialize)]
        struct Portrait {
            #[serde(rename = "px512x512")]
            img: String,
        }

        let path = format!("characters/{}/portrait", character_id);
        self
            .eve_client
            .fetch_oauth(&token, &path)
            .await?
            .json::<Portrait>()
            .await
            .map(|x| x.img)
            .map_err(Into::into)
    }

    pub async fn character(
        &self,
        token: &str,
        character_id: CharacterId,
    ) -> Result<Character, EveConnectError> {
        let path = format!("characters/{}/", character_id);
        self
            .eve_client
            .fetch_oauth(&token, &path)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    pub async fn assets(
        &self,
        token: &str,
        character_id: CharacterId,
    ) -> Result<Vec<CharacterAsset>, EveConnectError> {
        let path = format!("characters/{}/assets", character_id);
        self
            .eve_client
            .fetch_page_oauth::<CharacterAsset>(&token, &path)
            .await
            .map_err(Into::into)
    }

    pub async fn asset_names(
        &self,
        token: &str,
        character_id: CharacterId,
        ids: Vec<u64>,
    ) -> Result<Vec<CharacterAssetName>, EveConnectError> {
        let path = format!("characters/{}/assets/names", character_id);
        self
            .eve_client
            .post_oauth(&token, &path, &ids)
            .await
            .map_err(Into::into)
    }

    pub async fn blueprints(
        &self,
        token: &str,
        character_id: CharacterId,
    ) -> Result<Vec<CharacterBlueprint>, EveConnectError> {
        let path = format!("characters/{}/blueprints", character_id);
        self
            .eve_client
            .fetch_page_oauth::<CharacterBlueprint>(&token, &path)
            .await
            .map_err(Into::into)
    }

    pub async fn skills(
        &self,
        token: &str,
        character_id: CharacterId,
    ) -> Result<CharacterSkillRes, EveConnectError> {
        let path = format!("characters/{}/skills", character_id);
        self
            .eve_client
            .fetch_oauth(&token, &path)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    pub async fn skillqueue(
        &self,
        token: &str,
        character_id: CharacterId,
    ) -> Result<Vec<CharacterSkillQueue>, EveConnectError> {
        let path = format!("characters/{}/skillqueue", character_id);
        self
            .eve_client
            .fetch_oauth(&token, &path)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    pub async fn corporation_name(
        &self,
        cid: CorporationId,
    ) -> Result<String, EveConnectError> {
        #[derive(Deserialize)]
        struct Corp {
            name: String
        }

        let path = format!("corporations/{}", cid);
        self
            .eve_client
            .fetch(&path)
            .await?
            .json::<Corp>()
            .await
            .map(|x| x.name)
            .map_err(Into::into)
    }

    pub async fn alliance_name(
        &self,
        aid: AllianceId,
    ) -> Result<String, EveConnectError> {
        #[derive(Deserialize)]
        struct Alliance {
            name: String
        }

        let path = format!("alliances/{}", aid);
        self
            .eve_client
            .fetch(&path)
            .await?
            .json::<Alliance>()
            .await
            .map(|x| x.name)
            .map_err(Into::into)
    }

    pub async fn item_location(
        &self,
        token: &str,
        id:    u64,
    ) -> Result<Option<ItemLocation>, EveConnectError> {
        let path = format!("universe/structures/{}", id);
        self
            .eve_client
            .fetch_oauth(&token, &path)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    pub async fn whoami(
        &self,
        token: &str,
        character_id: u32,
    ) -> Result<String, EveConnectError> {
        #[derive(Deserialize)]
        struct Character {
            name: String,
        }

        let path = format!("characters/{}/", character_id);
        self
            .eve_client
            .fetch_oauth(&token, &path)
            .await?
            .json::<Character>()
            .await
            .map(|x| x.name)
            .map_err(Into::into)
    }

    pub async fn fitting(
        &self,
        token: &str,
        character_id: CharacterId,
    ) -> Result<Vec<CharacterFitting>, EveConnectError> {
        let path = format!("characters/{}/fittings", character_id);
        self
            .eve_client
            .fetch_oauth(&token, &path)
            .await?
            .json::<Vec<CharacterFitting>>()
            .await
            .map_err(Into::into)
    }
}

#[derive(Deserialize)]
pub struct Character {
    pub alliance_id:    Option<AllianceId>,
    pub corporation_id: CorporationId,
    pub name:           String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterAsset {
    pub is_singleton: bool,
    pub item_id: ItemId,
    pub location_flag: String,
    pub location_id: LocationId,
    pub location_type: String,
    pub quantity: u32,
    pub type_id: TypeId,

    pub is_blueprint_copy: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterAssetName {
    pub item_id: ItemId,
    pub name:    String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterBlueprint {
    pub item_id: ItemId,
    pub location_flag: String,
    pub location_id: LocationId,
    pub material_efficiency: u32,
    /// A range of numbers with a minimum of -2 and no maximum value where -1
    /// is an original and -2 is a copy. It can be a positive integer if it is
    /// a stack of blueprint originals fresh from the market (e.g. no 
    /// activities performed on them yet).
    pub quantity: i32,
    /// Number of runs remaining if the blueprint is a copy, -1 if it is an original
    pub runs: i32,
    pub time_efficiency: u32,
    pub type_id: TypeId,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CharacterSkillRes {
    pub skills:         Vec<CharacterSkill>,
    pub total_sp:       u64,
    pub unallocated_sp: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CharacterSkill {
    pub active_skill_level:   u32,
    pub skill_id:             u32,
    pub skillpoints_in_skill: u64,
    pub trained_skill_level:  u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterSkillQueue {
    pub finished_level:    u32,
    pub queue_position:    u32,
    pub skill_id:          u32,

    pub finish_date:       Option<String>,
    pub level_end_sp:      Option<u32>,
    pub level_start_sp:    Option<u32>,
    pub start_date:        Option<String>,
    pub training_start_sp: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemLocation {
    pub name:      String,
    pub owner_id:  u32,
    #[serde(alias = "solar_system_id")]
    pub system_id: SolarSystemId,
    pub type_id:   TypeId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterFitting {
    pub description:  String,
    pub fitting_id:   FittingId,
    pub items:        Vec<CharacterFittingItem>,
    pub name:         String,
    pub ship_type_id: TypeId
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterFittingItem {
    pub flag:     String,
    pub quantity: u32,
    pub type_id:  TypeId,
}
