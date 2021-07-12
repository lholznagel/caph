use async_trait::*;
use caph_eve_data_wrapper::{CharacterBlueprint, CharacterId, ItemId, LocationId, TypeId};
use cachem::{Parse, v2::{Cache, Command, Del, Get, Key, Set, Save}};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::BufStream;
use tokio::net::TcpStream;
use tokio::sync::{RwLock, watch::Receiver};

type Idx = ItemId;
type Val = CharacterBlueprintEntry;
type Typ = HashMap<Idx, Val>;

#[derive(Clone)]
pub struct CharacterBlueprintCache {
    cache: Arc<RwLock<Typ>>,
    cnc:   Receiver<Command>,
}

impl CharacterBlueprintCache {
    pub fn new(cnc: Receiver<Command>) -> Self {
        Self {
            cache: Arc::new(RwLock::default()),
            cnc,
        }
    }
}

impl Into<Arc<Box<dyn Cache>>> for CharacterBlueprintCache {
    fn into(self) -> Arc<Box<dyn Cache>> {
        Arc::new(Box::new(self))
    }
}

#[async_trait]
impl Cache for CharacterBlueprintCache {
    fn name(&self) -> String {
        "character_blueprint".into()
    }

    async fn handle(&self, cmd: Command, buf: &mut BufStream<TcpStream>) {
        match cmd {
            Command::Del => {
                let key = Idx::read(buf).await.unwrap();
                self.del(key).await;
                0u8.write(buf).await.unwrap();
            }
            Command::MDel => {
                let keys = Vec::<Idx>::read(buf).await.unwrap();
                self.mdel(keys).await;
                0u8.write(buf).await.unwrap();
            }
            Command::Get => {
                let key = Idx::read(buf).await.unwrap();
                let val = self.get(key, None).await;
                val.write(buf).await.unwrap();
            }
            Command::MGet => {
                let keys = Vec::<Idx>::read(buf).await.unwrap();
                let vals = self.mget(keys, None).await;
                vals.write(buf).await.unwrap();
            }
            Command::Keys => {
                self.keys().await.write(buf).await.unwrap();
            }
            Command::Set => {
                let key = Idx::read(buf).await.unwrap();
                let val = Val::read(buf).await.unwrap();
                self.set(key, val).await;
                self.save().await;
                0u8.write(buf).await.unwrap();
            }
            Command::MSet => {
                let vals = HashMap::<Idx, Val>::read(buf).await.unwrap();
                self.mset(vals).await;
                self.save().await;
                0u8.write(buf).await.unwrap();
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
impl Del for CharacterBlueprintCache {
    type Idx = Idx;

    async fn del(&self, idx: Self::Idx) {
        self
            .cache
            .write()
            .await
            .remove(&idx);
    }
}

#[async_trait]
impl Get for CharacterBlueprintCache {
    type Idx =   Idx;
    type Res =   Val;
    type Param = ();

    async fn get(&self, idx: Self::Idx, _: Option<Self::Param>) -> Option<Self::Res> {
        self
            .cache
            .read()
            .await
            .get(&idx)
            .cloned()
    }
}

#[async_trait]
impl Set for CharacterBlueprintCache {
    type Idx = Idx;
    type Val = Val;

    async fn set(&self, idx: Self::Idx, val: Self::Val) {
        self
            .cache
            .write()
            .await
            .insert(idx, val);
    }
}

#[async_trait]
impl Key for CharacterBlueprintCache {
    type Idx = Idx;

    async fn keys(&self) -> Vec<Self::Idx> {
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
impl Save for CharacterBlueprintCache {
    type Typ = Typ;

    fn file(&self) -> &str {
        "./db/character_blueprint.cachem"
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
pub struct CharacterBlueprintEntry {
    pub item_id:             ItemId,
    pub location_flag:       String,
    pub location_id:         LocationId,
    pub material_efficiency: u32,
    /// A range of numbers with a minimum of -2 and no maximum value where -1
    /// is an original and -2 is a copy. It can be a positive integer if it is
    /// a stack of blueprint originals fresh from the market (e.g. no 
    /// activities performed on them yet).
    pub quantity:            i32,
    /// Number of runs remaining if the blueprint is a copy, -1 if it is an original
    pub runs:                i32,
    pub time_efficiency:     u32,
    pub type_id:             TypeId,
    pub user_id:             CharacterId,
}

impl CharacterBlueprintEntry {
    pub fn from(x: CharacterBlueprint, user_id: CharacterId) -> Self {
        Self {
            item_id:             x.item_id,
            location_flag:       x.location_flag,
            location_id:         x.location_id,
            material_efficiency: x.material_efficiency,
            quantity:            x.quantity,
            runs:                x.runs,
            time_efficiency:     x.time_efficiency,
            type_id:             x.type_id,
            user_id
        }
    }
}

