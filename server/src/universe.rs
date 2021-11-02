use crate::ServerError;

use caph_connector::{CharacterId, StationId, SystemId};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone)]
pub struct UniverseService {
    pool: PgPool
}

impl UniverseService {
    /// Creates a new service instance
    ///
    /// # Params
    ///
    /// * `pool` -> Postgres connection pool
    ///
    /// # Returns
    ///
    /// New instance
    ///
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    /// Gets a list of all stations that where added by the character
    ///
    /// # Params
    ///
    /// * `cid` -> [CharacterId] of the character that added the station
    ///
    /// # Errors
    ///
    /// Fails if the postgres is not available
    ///
    /// # Returns
    ///
    /// List of stations
    ///
    pub async fn stations(
        &self,
        cid: CharacterId,
    ) -> Result<Vec<Station>, ServerError> {
        let entries = sqlx::query!("
                SELECT id, system_id, name, pos, structure
                FROM station
                WHERE character_id = $1
            ",
                *cid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                Station {
                    id:        x.id.into(),
                    system_id: x.system_id.into(),
                    name:      x.name,
                    structure: x.structure,
                    pos:       Some(x.pos)
                }
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    pub async fn station(
        &self,
        cid: CharacterId,
        sid: StationId,
    ) -> Result<Option<Station>, ServerError> {
        let entry = sqlx::query!("
                SELECT id, system_id, name, pos, character_id, structure
                FROM station
                WHERE id = $1
            ",
                *sid
            )
            .fetch_optional(&self.pool)
            .await?;
        if let Some(e) = entry {
            if e.pos && e.character_id != Some(*cid) {
                return Err(ServerError::NotFound);
            }

            Ok(Some(Station {
                id:        e.id.into(),
                system_id: e.system_id.into(),
                name:      e.name,
                structure: e.structure,
                pos:       Some(e.pos)
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_station(
        &self,
        cid: CharacterId,
        sid: StationId,
    ) -> Result<(), ServerError> {
        sqlx::query!("
                DELETE FROM station
                WHERE character_id = $1
                  AND id = $2
                  AND pos = TRUE
            ",
                *cid,
                *sid
            )
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn add_station(
        &self,
        cid:     CharacterId,
        station: Station
    ) -> Result<(), ServerError> {
        sqlx::query!("
                INSERT INTO station (character_id, id, system_id, name, structure, pos)
                VALUES ($1, $2, $3, $4, $5, TRUE)
            ",
                *cid,
                *station.id,
                *station.system_id,
                &station.name,
                &station.structure,
            )
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn systems(
        &self,
    ) -> Result<Vec<System>, ServerError> {
        let entries = sqlx::query!("
                SELECT id, name
                FROM system
            ")
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                System {
                    id:          x.id.into(),
                    name:        x.name,
                }
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    pub async fn system(
        &self,
        sid: SystemId,
    ) -> Result<Option<System>, ServerError> {
        let entry = sqlx::query!("
                SELECT id, name
                FROM system
                WHERE id = $1
            ",
                *sid
            )
            .fetch_optional(&self.pool)
            .await?;
        if let Some(e) = entry {
            Ok(Some(System {
                id:          e.id.into(),
                name:        e.name,
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Station {
    pub id:        StationId,
    pub system_id: SystemId,
    pub name:      String,
    pub structure: String,
    pub pos:       Option<bool>
}

#[derive(Debug, Serialize)]
pub struct System {
    pub id:   SystemId,
    pub name: String,
}
