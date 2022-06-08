use crate::{AuthService, EVE_DEFAULT_SCOPE};
use crate::error::Error;

use caph_connector::{AllianceId, CharacterId, ConnectCharacterService, CorporationId, EveAuthClient, BlueprintEntry, EveClient, TypeId, CorporationService};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct CharacterService {
    pool: PgPool,

    auth_service: AuthService
}

impl CharacterService {
    pub fn new(
        pool: PgPool,

        auth_service: AuthService
    ) -> Self {
        Self {
            pool,

            auth_service
        }
    }

    /// Gets a list of all character ids that are associated with the given
    /// character id
    ///
    /// # Params
    ///
    /// * `cid` - Logged in character id
    ///
    /// # Error
    ///
    /// Failes when there is a database problem
    ///
    /// # Returns
    ///
    /// List of all character ids
    ///
    pub async fn ids(
        &self,
        cid: CharacterId
    ) -> Result<Vec<i32>, Error> {
        let ids = sqlx::query!("
                SELECT character_id
                FROM   characters
                WHERE  character_id = $1 OR character_main = $1
            ", *cid as i32)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| x.character_id)
            .collect::<Vec<_>>();
        Ok(ids)
    }

    /// Gets a list of alts for the given [CharacterId]
    ///
    /// # Params
    ///
    /// * `cid` -> [CharacterId] of the requesting character
    ///
    /// # Returns
    ///
    /// List of alt characters
    ///
    pub async fn alts(
        &self,
        cid:    CharacterId
    ) -> Result<Vec<Character>, Error> {
        let alts = sqlx::query!("
                SELECT DISTINCT character_id
                FROM logins
                WHERE character_main = $1
                  AND character_id IS NOT NULL
            ", *cid as i32)
            .fetch_all(&self.pool)
            .await?;

        let mut characters = Vec::new();
        for alt in alts {
            let character_id = alt.character_id;

            if let Some(x) = character_id {
                let character_id = x.into();
                let character = self.fetch_info(
                    character_id,
                    Some(cid)
                ).await?;
                characters.push(character);
            }
        }

        Ok(characters)
    }

    /// Gets a list of alts for the given [CharacterId]
    ///
    /// # Params
    ///
    /// * `cid` -> [CharacterId] of the requesting character
    ///
    /// # Returns
    ///
    /// List of alt characters
    ///
    pub async fn blueprints(
        &self,
        cid: CharacterId
    ) -> Result<Vec<BlueprintEntry>, Error> {
        let character_ids = self
            .alts(cid)
            .await?
            .into_iter()
            .map(|x| x.character_id)
            .collect::<Vec<_>>();

        let mut blueprints = Vec::new();
        for cid in character_ids {
            let refresh_token = self
                .auth_service
                .refresh_token(&cid)
                .await?;

            let client = EveAuthClient::new(refresh_token)?;
            let character_service = ConnectCharacterService::new(cid);
            let character_bps = character_service.blueprints(&client).await?;
            blueprints.extend(character_bps);
        }

        Ok(blueprints)
    }

    pub async fn corporation_blueprints(
        &self,
        cid: CharacterId
    ) -> Result<Vec<BlueprintEntry>, Error> {
        let corporation_id = sqlx::query!("
                SELECT DISTINCT corporation_id, character_id
                FROM characters
                WHERE (
                       character_main = $1
                    OR character_id   = $1
                )
                AND 'esi-corporations.read_blueprints.v1' = ANY(esi_tokens)
            ", *cid as i32)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| (x.corporation_id.into(), x.character_id.into()))
            .collect::<Vec<_>>();

        let mut blueprints = Vec::new();
        for (corp_id, char_id) in corporation_id {
            let refresh_token = self
                .auth_service
                .refresh_token(&char_id)
                .await?;

            let client = EveAuthClient::new(refresh_token)?;
            let corporation_service = CorporationService::new(corp_id);
            let corporation_bps = corporation_service
                .blueprints(&client)
                .await?;
            blueprints.extend(corporation_bps);
        }

        Ok(blueprints)
    }

    pub async fn remove(
        &self,
        cid: CharacterId
    ) -> Result<(), Error> {
        sqlx::query!("
                DELETE FROM characters WHERE character_id = $1
            ",
                *cid
            )
            .execute(&self.pool)
            .await?;
        sqlx::query!("
                DELETE FROM logins WHERE character_id = $1
            ",
                *cid
            )
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Fetches information either from the database or the eve servers
    ///
    /// # Params
    ///
    /// `cid`  -> Character id of the character to fetch
    /// `main` -> Optional character id of the main account
    ///
    /// # Returns
    ///
    /// Alliance, character and corporation information
    ///
    pub async fn info(
        &self,
        cid: CharacterId,
    ) -> Result<Option<Character>, Error> {
        // FIXME: esi_tokens must be set as NOT NULL in SQL
        let character = sqlx::query!(r#"
                SELECT
                    c.alliance_id,
                    c.alliance_name,
                    c.character_id,
                    c.character_name,
                    c.corporation_id,
                    c.corporation_name,
                    c.esi_tokens       AS "esi_tokens!"
                FROM logins l
                JOIN characters c
                ON l.character_id = c.character_id
                WHERE c.character_id = $1;
            "#, *cid as i32)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(x) = character {
            let character = Character::new(
                x.alliance_name,
                x.alliance_id.map(|x| x.into()),
                x.character_name,
                x.character_id.into(),
                x.corporation_name,
                x.corporation_id.into(),
                x.esi_tokens
            );
            Ok(Some(character))
        } else {
            Ok(None)
        }
    }

    pub async fn fetch_info(
        &self,
        cid:  CharacterId,
        main: Option<CharacterId>,
    ) -> Result<Character, Error> {
        if let Some(x) = self.info(cid).await? {
            Ok(x)
        } else {
            let character = self.eve_character_info(cid).await?;
            self.save(&character, main).await?;
            Ok(character)
        }
    }

    pub async fn refresh(
        &self,
        cid: CharacterId
    ) -> Result<(), Error> {
        let character = self.eve_character_info(cid).await?;

        let main = sqlx::query!("
                SELECT
                    character_main
                FROM characters
                WHERE character_id = $1
            ",
                *cid
            )
            .fetch_one(&self.pool)
            .await?;
        if let Some(x) = main.character_main {
            self.save(&character, Some(x.into())).await?;
        } else {
            self.save(&character, None).await?;
        }

        Ok(())
    }

    /// Saves the character information in the database
    ///
    /// # Params
    ///
    /// * `character` -> All information about the character
    /// * `main`      -> Optional main character id
    ///
    async fn save(
        &self,
        character: &Character,
        main:      Option<CharacterId>
    ) -> Result<(), Error> {
        sqlx::query!("
                INSERT INTO characters
                (
                    alliance_id, alliance_name,
                    character_id, character_name,
                    corporation_id, corporation_name,
                    character_main,
                    esi_tokens
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT (character_id)
                DO UPDATE SET
                    alliance_id      = $1,
                    alliance_name    = $2,
                    corporation_id   = $5,
                    corporation_name = $6
            ",
            character.alliance_id.map(|x| *x),
            character.alliance,
            *character.character_id as i32,
            character.character,
            *character.corporation_id as i32,
            character.corporation,
            main.map(|x| *x as i32),
            &character.esi_tokens,
        )
        .execute(&self.pool)
        .await
        .map(drop)
        .map_err(Into::into)
    }

    /// Fetches all character infos from eve
    ///
    /// # Params
    ///
    /// * `cid` -> Character id to fetch
    ///
    /// # Returns
    ///
    /// Alliance, character and corporation information
    ///
    async fn eve_character_info(
        &self,
        cid: CharacterId,
    ) -> Result<Character, Error> {
        let client = EveClient::new()?;
        let character_service = ConnectCharacterService::new(cid);
        let character = character_service.info(&client).await?;

        let aid = character.alliance_id;
        let alliance = if let Some(x) = aid {
            Some(character_service.alliance_name(&client, x).await?)
        } else {
            None
        };

        let coid = character.corporation_id;
        let corporation = character_service.corporation_name(&client, coid).await?;

        Ok(Character::new(
            alliance,
            aid,
            character.name,
            cid,
            corporation,
            coid,
            EVE_DEFAULT_SCOPE
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        ))
    }
}

/// Represents a character with all its information
#[derive(Debug, Serialize)]
pub struct Character {
    pub alliance:         Option<String>,
    pub alliance_icon:    Option<String>,
    pub alliance_id:      Option<AllianceId>,
    pub character:        String,
    pub character_id:     CharacterId,
    pub character_icon:   String,
    pub corporation:      String,
    pub corporation_icon: String,
    pub corporation_id:   CorporationId,
    pub esi_tokens:       Vec<String>
}

impl Character {
    pub fn new(
        alliance:       Option<String>,
        alliance_id:    Option<AllianceId>,
        character:      String,
        character_id:   CharacterId,
        corporation:    String,
        corporation_id: CorporationId,
        esi_tokens:     Vec<String>
    ) -> Self {
        let alliance_icon = if let Some(x) = alliance_id {
            Some(format!("https://images.evetech.net/alliances/{}/logo?size=1024", x))
        } else {
            None
        };

        Self {
            alliance,
            alliance_id,
            alliance_icon,
            character,
            character_id,
            character_icon: format!(
                "https://images.evetech.net/characters/{}/portrait?size=1024",
                character_id
            ),
            corporation,
            corporation_id,
            corporation_icon: format!(
                "https://images.evetech.net/corporations/{}/logo?size=1024",
                corporation_id
            ),
            esi_tokens
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct BlueprintTotalEntry {
    pub stored:  bool,

    pub me:      Option<i32>,
    pub te:      Option<i32>,

    pub name:    String,
    pub type_id: TypeId,
    pub price:   f64,
}

impl BlueprintTotalEntry {
    pub fn new(
        name:    String,
        type_id: TypeId,
        price:   f64
    ) -> Self {
        Self {
            stored:  false,
            me:      None,
            te:      None,
            name:    name,
            type_id: type_id,
            price:   price
        }
    }
}
