use async_trait::*;
use caph_eve_data_wrapper::{BlueprintAdditional, BlueprintMaterial, BlueprintSkill, TypeId};
use cachem::{Parse, Cache, Command, Get, Key, Set, Save};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::BufStream;
use tokio::net::TcpStream;
use tokio::sync::{RwLock, watch::Receiver};

type Id  = TypeId;
type Val = BlueprintEntry;
type Typ = HashMap<Id, Val>;

pub struct BlueprintCache {
    cache: RwLock<Typ>,
    cnc:   Receiver<Command>,
}

impl BlueprintCache {
    pub fn new(cnc: Receiver<Command>) -> Self {
        Self {
            cache: RwLock::default(),
            cnc,
        }
    }
}

impl Into<Arc<dyn Cache>> for BlueprintCache {
    fn into(self) -> Arc<dyn Cache> {
        Arc::new(self)
    }
}

#[async_trait]
impl Cache for BlueprintCache {
    fn name(&self) -> String {
        "blueprints".into()
    }

    async fn handle(&self, cmd: Command, buf: &mut BufStream<TcpStream>) {
        match cmd {
            Command::Get => {
                let key = Id::read(buf).await.unwrap();
                let val = self.get(key, None).await;
                val.write(buf).await.unwrap();
            }
            Command::MGet => {
                let keys = Vec::<Id>::read(buf).await.unwrap();
                let vals = self.mget(keys, None).await;
                vals.write(buf).await.unwrap();
            }
            Command::Set => {
                let key = Id::read(buf).await.unwrap();
                let val = Val::read(buf).await.unwrap();
                self.set(key, val).await;
                self.save().await;
                0u8.write(buf).await.unwrap();
            }
            Command::MSet => {
                let vals = HashMap::<Id, Val>::read(buf).await.unwrap();
                self.mset(vals).await;
                self.save().await;
                0u8.write(buf).await.unwrap();
            }
            Command::Keys => {
                self.keys().await.write(buf).await.unwrap();
            }
            _ => {
                log::error!("Invalid cmd {:?}", cmd);
            }
        }
    }

    async fn cnc_listener(&self) {
        let mut cnc_copy = self.cnc.clone();
        loop {
            cnc_copy.changed().await.unwrap();
            let cmd = *cnc_copy.borrow();

            match cmd {
                Command::Save => { self.save().await; },
                _ => { log::warn!("Invalid cmd send over cnc: {:?}", cmd); }
            }
        }
    }
}

#[async_trait]
impl Get for BlueprintCache {
    type Id  =   Id;
    type Res =   Val;
    type Param = ();

    async fn get(&self, id: Self::Id, _: Option<Self::Param>) -> Option<Self::Res> {
        self
            .cache
            .read()
            .await
            .get(&id)
            .cloned()
    }
}

#[async_trait]
impl Set for BlueprintCache {
    type Id  = Id;
    type Val = Val;

    async fn set(&self, id: Self::Id, val: Self::Val) {
        self
            .cache
            .write()
            .await
            .insert(id, val);
    }
}

#[async_trait]
impl Key for BlueprintCache {
    type Id = Id;

    async fn keys(&self) -> Vec<Self::Id> {
        self
            .cache
            .read()
            .await
            .keys()
            .map(|x| *x)
            .collect::<Vec<_>>()
    }
}

#[async_trait]
impl Save for BlueprintCache {
    type Typ = Typ;

    fn file(&self) -> &str {
        "./db/blueprints.cachem"
    }

    async fn read(&self) -> Self::Typ {
        self.cache.read().await.clone()
    }

    async fn write(&self, data: Self::Typ) {
        *self.cache.write().await = data;
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct BlueprintEntry {
    pub bid:           TypeId,
    pub copy:          Option<Activity>,
    pub invention:     Option<Activity>,
    pub manufacture:   Option<Activity>,
    pub reaction:      Option<Activity>,
    pub research_mat:  Option<Activity>,
    pub research_time: Option<Activity>,
    pub limit:         u32,
}

impl BlueprintEntry {
    pub fn new(
        bid:           TypeId,
        copy:          Option<Activity>,
        invention:     Option<Activity>,
        manufacture:   Option<Activity>,
        reaction:      Option<Activity>,
        research_mat:  Option<Activity>,
        research_time: Option<Activity>,
        limit:         u32,
    ) -> Self {
        Self {
            bid,
            copy,
            invention,
            manufacture,
            reaction,
            research_mat,
            research_time,
            limit,
        }
    }
}

impl BlueprintEntry {
    /// Returns either a manufacture activity or a reaction activity
    pub fn production_activity(&self) -> Activity {
        if let Some(man) = &self.manufacture {
            man.clone()
        } else if let Some(reac) = &self.reaction {
            reac.clone()
        } else {
            // This should never every be the case
            log::error!("The blueprint {:?} has an invalid acitivity state", self.bid);
            Activity::default()
        }
    }
}

impl From<&caph_eve_data_wrapper::BlueprintEntry> for BlueprintEntry {
    fn from(x: &caph_eve_data_wrapper::BlueprintEntry) -> Self {
        let copy = if let Some(x) = &x.activities.copying {
            Some(Activity::from(x))
        } else { None };
        let invention = if let Some(x) = &x.activities.invention {
            Some(Activity::from(x))
        } else { None };
        let manufacture = if let Some(x) = &x.activities.manufacturing {
            Some(Activity::from(x))
        } else { None };
        let reaction = if let Some(x) = &x.activities.reaction {
            Some(Activity::from(x))
        } else { None };
        let research_mat = if let Some(x) = &x.activities.research_material {
            Some(Activity::from(x))
        } else { None };
        let research_time = if let Some(x) = &x.activities.research_time {
            Some(Activity::from(x))
        } else { None };

        Self {
            bid: x.type_id,
            copy,
            invention,
            manufacture,
            reaction,
            research_mat,
            research_time,
            limit: x.max_production_limit,
        }
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct Activity {
    pub materials: Option<Vec<Material>>,
    pub products:  Option<Vec<Material>>,
    pub skills:    Option<Vec<Skill>>,
    pub time:      u32,
}

impl Default for Activity {
    fn default() -> Self {
        Self {
            materials: None,
            products:  None,
            skills:    None,
            time:      0
        }
    }
}

impl Activity {
    /// Gets the `TypeId` of the product that this activity produces
    ///
    /// # Returns
    ///
    /// `TypeId` of the product. If there is no product `TypeId(0)` will
    /// returned
    ///
    pub fn product_id(&self) -> TypeId {
        if let Some(x) = self.products.as_ref() {
            x[0].mid
        } else {
            log::error!("Activity without product {:?}", self);
            TypeId(0)
        }
    }

    /// Returns the required materials to produce the product
    ///
    /// # Returns
    ///
    /// A vector of [Material]. If there are no materials, an empty vector is
    /// returned
    ///
    pub fn materials(&self) -> Vec<Material> {
        if let Some(x) = self.materials.as_ref() {
            x.to_vec()
        } else {
            log::error!("Activity without materials {:?}", self);
            Vec::new()
        }
    }

    /// Gets the list of required materials by their id
    ///
    /// # Returns
    ///
    /// Vector of [TypeId]
    ///
    pub fn material_ids(&self) -> Vec<TypeId> {
        self
            .materials()
            .into_iter()
            .map(|x| x.mid)
            .collect::<Vec<_>>()
    }
}

impl From<&BlueprintAdditional> for Activity {
    fn from(x: &BlueprintAdditional) -> Self {
        let materials = if let Some(x) = &x.materials {
            Some(x.iter().map(Material::from).collect::<Vec<_>>())
        } else { None };
        let products = if let Some(x) = &x.products {
            Some(x.iter().map(Material::from).collect::<Vec<_>>())
        } else { None };
        let skills = if let Some(x) = &x.skills {
            Some(x.iter().map(Skill::from).collect::<Vec<_>>())
        } else { None };

        Self {
            materials,
            products,
            skills,
            time: x.time
        }
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq, Parse)]
pub struct Material {
    pub mid:         TypeId,
    pub quantity:    u32,

    pub probability: Option<f32>,
}

impl Material {
    pub fn new(
        mid:         TypeId,
        quantity:    u32,
        probability: Option<f32>,
    ) -> Self {
        Self {
            mid,
            quantity,
            probability
        }
    }
}

impl From<&BlueprintMaterial> for Material {
    fn from(x: &BlueprintMaterial) -> Self {
        Self {
            mid:         x.type_id,
            quantity:    x.quantity,
            probability: x.probability,
        }
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct Skill {
    pub level:   u32,
    pub type_id: TypeId,
}

impl Skill {
    pub fn new(
        level:   u32,
        type_id: TypeId,
    ) -> Self {
        Self {
            level,
            type_id,
        }
    }
}

impl From<&BlueprintSkill> for Skill {
    fn from(x: &BlueprintSkill) -> Self {
        Self {
            level:   x.level,
            type_id: x.type_id
        }
    }
}
