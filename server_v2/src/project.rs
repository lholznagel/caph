use crate::blueprint::{BlueprintInfo, BlueprintService, BlueprintTreeEntry, ManufactureCost};
use crate::character::CharacterService;
use crate::error::EveServerError;
use crate::eve::EveAuthService;

use cachem::v2::ConnectionPool;
use caph_db_v2::{CacheName, CharacterAssetEntry, Material, ProjectBlueprintEntry, ProjectEntry};
use caph_eve_data_wrapper::{ItemId, SolarSystemId, TypeId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct ProjectService {
    pool:      ConnectionPool,
    blueprint: BlueprintService,
    character: CharacterService,
    eve_auth:  EveAuthService,
}

impl ProjectService {
    pub fn new(
        pool:      ConnectionPool,
        blueprint: BlueprintService,
        character: CharacterService,
        eve_auth:  EveAuthService,
    ) -> Self {
        Self {
            pool,
            blueprint,
            character,
            eve_auth,
        }
    }

    pub async fn all(
        &self,
        token: String,
    ) -> Result<Vec<ProjectEntry>, EveServerError> {
        let user_id = self
            .eve_auth
            .lookup(&token)
            .await?
            .ok_or(EveServerError::InvalidUser)?
            .user_id;

        let project_ids = self
            .pool
            .acquire()
            .await?
            .keys::<_, Uuid>(CacheName::Project)
            .await?;
        let projects = self
            .pool
            .acquire()
            .await?
            .mget::<_, _, ProjectEntry>(CacheName::Project, project_ids)
            .await?
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .filter(|x| x.user_id == user_id)
            .collect::<Vec<_>>();
        Ok(projects)
    }

    pub async fn by_id(
        &self,
        id:    Uuid,
        token: String,
    ) -> Result<Option<ProjectEntry>, EveServerError> {
        let user_id = self
            .eve_auth
            .lookup(&token)
            .await?
            .ok_or(EveServerError::InvalidUser)?
            .user_id;

        let project = self
            .pool
            .acquire()
            .await?
            .get::<_, _, ProjectEntry>(CacheName::Project, id)
            .await?
            .unwrap();
        if project.user_id == user_id {
            Ok(Some(project))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(
        &self,
        id:    Uuid,
        token: &str
    ) -> Result<(), EveServerError> {
        let _ = self
            .eve_auth
            .lookup(&token)
            .await?
            .ok_or(EveServerError::InvalidUser)?;

        self
            .pool
            .acquire()
            .await?
            .del(CacheName::Project, id)
            .await
            .map_err(Into::into)
    }

    pub async fn blueprints(
        &self,
        id:    Uuid,
        token: String,
    ) -> Result<Vec<Blueprint>, EveServerError> {
        let project = self.get_project(id, token.clone()).await?;

        let bpids = project
            .blueprints
            .into_iter()
            .map(|x| x.bpid)
            .collect::<Vec<_>>();
        let bps = self
            .blueprint
            .required_blueprints(bpids)
            .await?;
        let bps = self
            .blueprint
            .by_ids(bps)
            .await?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        let stored = self
            .stored_materials(id, token.clone())
            .await?
            .into_iter()
            .map(|x| (x.type_id, x))
            .collect::<HashMap<_, _>>();
        let character_bps = self
            .character
            .blueprints(token.clone())
            .await?
            .into_iter()
            .map(|x| (x.item_id, x))
            .collect::<HashMap<_, _>>();

        let mut result = Vec::new();
        for bp in bps {
            let (is_stored, item_id) = if let Some(e) = stored.get(&bp.bid) {
                (true, e.item_id)
            } else {
                (false, 0.into())
            };

            let product = bp.production_activity().product_id();
            let is_product_stored = stored.get(&product).is_some();

            let invention = if let Some(e) = bp.invention {
                stored.get(&e.product_id()).is_some()
            } else {
                false
            };

            let (mat_eff, time_eff) = if is_stored {
                if let Some(e) = self
                    .character
                    .blueprint_by_item_id(item_id)
                    .await? {
                    (Some(e.material_efficiency), Some(e.time_efficiency))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };

            result.push(Blueprint {
                bpid:    bp.bid,
                stored:  is_stored,
                product: is_product_stored,
                invention,
                mat_eff,
                time_eff,
            });
        }

        Ok(result)
    }

    pub async fn create(
        &self,
        body:  ProjectNew,
        token: String,
    ) -> Result<Uuid, EveServerError> {
        let user_id = self
            .eve_auth
            .lookup(&token)
            .await?
            .ok_or(EveServerError::InvalidUser)?
            .user_id;

        let id = Uuid::new_v4();
        let project = ProjectEntry {
            id,
            name:       body.name,
            system:     body.system,
            chest:      body.chest,
            blueprints: body.blueprints,
            user_id,
        };

        self
            .pool
            .acquire()
            .await?
            .set(CacheName::Project, id, project)
            .await?;

        Ok(id)
    }

    pub async fn cost(
        &self,
        id:    Uuid,
        token: String,
    ) -> Result<Vec<ManufactureCost>, EveServerError> {
        let project = self.get_project(id, token).await?;

        let bpids = project
            .blueprints
            .into_iter()
            .map(|x| (x.bpid, x.runs))
            .collect::<HashMap<_, _>>();
        let sid = project.system;

        self
            .blueprint
            .manufacture_cost(bpids, sid)
            .await
    }

    pub async fn materials(
        &self,
        id:    Uuid,
        token: String,
    ) -> Result<Vec<Material>, EveServerError> {
        let project = self.get_project(id, token).await?;

        let bpids = project
            .blueprints
            .into_iter()
            .map(|x| BlueprintInfo {
                bpid: x.bpid,
                runs: x.runs
            })
            .collect::<Vec<_>>();

        self
            .blueprint
            .materials(bpids)
            .await
    }

    pub async fn raw_materials(
        &self,
        id:    Uuid,
        token: String
    ) -> Result<Vec<Material>, EveServerError> {
        let project = self.get_project(id, token).await?;

        let bpids = project
            .blueprints
            .into_iter()
            .map(|x| BlueprintInfo {
                bpid: x.bpid,
                runs: x.runs
            })
            .collect::<Vec<_>>();

        self
            .blueprint
            .raw_materials(bpids)
            .await
    }

    pub async fn stored_materials(
        &self,
        id:    Uuid,
        token: String
    ) -> Result<Vec<CharacterAssetEntry>, EveServerError> {
        let project_chest = self
            .get_project(id, token)
            .await?
            .chest;

        let keys = self
            .pool
            .acquire()
            .await?
            .keys::<_, ItemId>(CacheName::CharacterAsset)
            .await?;
        let assets = self
            .pool
            .acquire()
            .await?
            .mget::<_, _, CharacterAssetEntry>(CacheName::CharacterAsset, keys)
            .await?
            .into_iter()
            .flatten()
            .filter(|x| x.location_id == (*project_chest).into())
            .collect::<Vec<_>>();
        Ok(assets)
    }

    pub async fn trees(
        &self,
        id:    Uuid,
        token: String,
    ) -> Result<Vec<BlueprintTreeEntry>, EveServerError> {
        let project = self.get_project(id, token).await?;

        let bpids = project
            .blueprints
            .into_iter()
            .map(|x| x.bpid)
            .collect::<Vec<_>>();

        self
            .blueprint
            .tree(bpids)
            .await
    }

    pub async fn manufacture(
        &self,
        id:    Uuid,
        token: String,
    ) -> Result<Vec<RequiredProducts>, EveServerError> {
        let project = self.get_project(id, token.clone()).await?;
        let bpids = project
            .blueprints
            .into_iter()
            .map(|x| BlueprintInfo {
                bpid: x.bpid,
                runs: x.runs
            })
            .collect::<Vec<_>>();

        let stored_assets = self
            .stored_materials(id, token)
            .await?
            .into_iter()
            .map(|x| (x.type_id, x))
            .collect::<HashMap<_, _>>();

        let products = self
            .blueprint
            .manufacture(bpids)
            .await?;

        let mut required = Vec::new();
        for product in products {
            if let Some(e) = stored_assets.get(&product.pid) {
                let mut materials = Vec::new();
                for material in product.materials {
                    if let Some(e) = stored_assets.get(&material.mid) {
                        materials.push(RequiredProductsMaterial {
                            mid:      material.mid,
                            quantity: material.quantity,
                            stored:   e.quantity
                        });
                    } else {
                        materials.push(RequiredProductsMaterial {
                            mid:      material.mid,
                            quantity: material.quantity,
                            stored:   0
                        });
                    }
                }

                required.push(RequiredProducts {
                    pid:       product.pid,
                    bpid:      product.bpid,
                    quantity:  product.quantity,
                    stored:    e.quantity,
                    depth:     product.depth,
                    materials,
                });
            } else {
                let mut materials = Vec::new();
                for material in product.materials {
                    if let Some(e) = stored_assets.get(&material.mid) {
                        materials.push(RequiredProductsMaterial {
                            mid:      material.mid,
                            quantity: material.quantity,
                            stored:   e.quantity
                        });
                    } else {
                        materials.push(RequiredProductsMaterial {
                            mid:      material.mid,
                            quantity: material.quantity,
                            stored:   0
                        });
                    }
                }

                required.push(RequiredProducts {
                    pid:       product.pid,
                    bpid:      product.bpid,
                    quantity:  product.quantity,
                    stored:    0,
                    depth:     product.depth,
                    materials,
                });
            }
        }

        Ok(required)
    }

    async fn get_project(
        &self,
        id: Uuid,
        token: String
    ) -> Result<ProjectEntry, EveServerError> {
        let user_id = self
            .eve_auth
            .lookup(&token)
            .await?
            .ok_or(EveServerError::InvalidUser)?
            .user_id;

        let project = self
            .pool
            .acquire()
            .await?
            .get::<_, _, ProjectEntry>(CacheName::Project, id)
            .await?
            .unwrap();
        if project.user_id != user_id {
            Err(EveServerError::InvalidUser)
        } else {
            Ok(project)
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectNew {
    name:       String,
    system:     SolarSystemId,
    chest:      ItemId,
    blueprints: Vec<ProjectBlueprintEntry>,
}

#[derive(Debug, Serialize)]
pub struct RequiredProducts {
    pub pid:       TypeId,
    pub bpid:      TypeId,
    pub quantity:  u32,
    pub stored:    u32,
    pub materials: Vec<RequiredProductsMaterial>,
    pub depth:     u8,
}

#[derive(Debug, Serialize)]
pub struct RequiredProductsMaterial {
    pub mid:      TypeId,
    pub quantity: u32,
    pub stored:   u32,
}

#[derive(Debug, Serialize)]
pub struct Blueprint {
    pub bpid:      TypeId,
    pub stored:    bool,
    pub product:   bool,
    pub invention: bool,
    pub mat_eff:   Option<u32>,
    pub time_eff:  Option<u32>,
}
