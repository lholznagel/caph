use crate::error::ServerError;

use caph_connector::{AllianceId, CharacterId, ConnectCharacterService, CorporationId, EveAuthClient};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Clone)]
pub struct CharacterService {
    pool: PgPool
}

impl CharacterService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
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
    pub async fn by_id(
        &self,
        cid: CharacterId
    ) -> Result<Character, ServerError> {
        let character = sqlx::query!("
                SELECT *
                FROM characters
                WHERE character_id = $1
            ", *cid as i32)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(c) = character {
            let character = Character::new(
                c.alliance_name,
                c.alliance_id.map(|x| x.into()),
                c.character_name,
                c.character_id.into(),
                c.corporation_name,
                c.corporation_id.into()
            );
            Ok(character)
        } else {
            Err(ServerError::InvalidUser)
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
    ) -> Result<Vec<i32>, ServerError> {
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
        client: EveAuthClient,
        cid:    CharacterId
    ) -> Result<Vec<Character>, ServerError> {
        let alts = sqlx::query!("
                SELECT DISTINCT character_id
                FROM logins
                WHERE character_main = $1 AND character_id IS NOT NULL
            ", *cid as i32)
            .fetch_all(&self.pool)
            .await?;

        let mut characters = Vec::new();
        for alt in alts {
            let character_id = alt.character_id;

            if let Some(x) = character_id {
                let character_id = x.into();
                let character = self.info(client.clone(), character_id, Some(cid)).await?;
                characters.push(character);
            }
        }

        Ok(characters)
    }

    pub async fn remove(
        &self,
        cid: CharacterId
    ) -> Result<(), ServerError> {
        sqlx::query!("
                DELETE FROM characters WHERE character_id = $1
            ",
                *cid
            )
            .execute(&self.pool)
            .await?;
        sqlx::query!("
                DELETE FROM logins
                WHERE character_id = $1
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
        client: EveAuthClient,
        cid:    CharacterId,
        main:   Option<CharacterId>
    ) -> Result<Character, ServerError> {
        let character = sqlx::query!("
            SELECT
                c.alliance_id,
                c.alliance_name,
                c.character_id,
                c.character_name,
                c.corporation_id,
                c.corporation_name
            FROM logins l
            JOIN characters c
            ON l.character_id = c.character_id
            WHERE c.character_id = $1;
        ", *cid as i32)
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
            );
            Ok(character)
        } else {
            let character = self.eve_character_info(client, cid).await?;
            self.save(&character, main).await?;
            Ok(character)
        }
    }

    pub async fn refresh(
        &self,
        cid: CharacterId
    ) -> Result<(), ServerError> {
        let code = sqlx::query!("
                SELECT
                    refresh_token
                FROM logins
                WHERE character_id = $1
            ",
                * cid
            )
            .fetch_one(&self.pool)
            .await?;
        if let Some(x) = code.refresh_token {
            let client = EveAuthClient::new(x)?;
            let character = self.eve_character_info(client, cid).await?;

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
    ) -> Result<(), ServerError> {
        sqlx::query!("
                INSERT INTO characters
                (
                    alliance_id, alliance_name,
                    character_id, character_name,
                    corporation_id, corporation_name,
                    character_main
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
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
            main.map(|x| *x as i32)
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
        client: EveAuthClient,
        cid:    CharacterId,
    ) -> Result<Character, ServerError> {
        let character_service = ConnectCharacterService::new(&client, cid);

        let character = character_service.info().await?;

        let aid = character.alliance_id;
        let alliance = if let Some(x) = aid {
            Some(character_service.alliance_name(x).await?)
        } else {
            None
        };

        let coid = character.corporation_id;
        let corporation = character_service.corporation_name( coid).await?;

        Ok(Character::new(
            alliance,
            aid,
            character.name,
            cid,
            corporation,
            coid
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
}

impl Character {
    pub fn new(
        alliance:       Option<String>,
        alliance_id:    Option<AllianceId>,
        character:      String,
        character_id:   CharacterId,
        corporation:    String,
        corporation_id: CorporationId
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
            character_id: character_id.into(),
            character_icon: format!(
                "https://images.evetech.net/characters/{}/portrait?size=1024",
                character_id
            ),
            corporation,
            corporation_id: corporation_id.into(),
            corporation_icon: format!(
                "https://images.evetech.net/corporations/{}/logo?size=1024",
                corporation_id
            ),
        }
    }
}

