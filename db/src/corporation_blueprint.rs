use async_trait::*;
use caph_eve_data_wrapper::{CharacterId, CorporationId, LocationId, TypeId};
use cachem::{Parse, Cache, Command, Del, Get, Key, Set, Save};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::BufStream;
use tokio::net::TcpStream;
use tokio::sync::{RwLock, watch::Receiver};

type Id  = Uuid;
type Val = CorporationBlueprintEntry;
type Typ = HashMap<Id, Val>;

#[derive(Clone)]
pub struct CorporationBlueprintCache {
    cache: Arc<RwLock<Typ>>,
    cnc:   Receiver<Command>,
}

impl CorporationBlueprintCache {
    pub fn new(cnc: Receiver<Command>) -> Self {
        Self {
            cache: Arc::new(RwLock::default()),
            cnc,
        }
    }
}

impl Into<Arc<dyn Cache>> for CorporationBlueprintCache {
    fn into(self) -> Arc<dyn Cache> {
        Arc::new(self)
    }
}

#[async_trait]
impl Cache for CorporationBlueprintCache {
    fn name(&self) -> String {
        "corporation_blueprint".into()
    }

    async fn handle(&self, cmd: Command, buf: &mut BufStream<TcpStream>) {
        match cmd {
            Command::MDel => {
                let keys = Vec::<Id>::read(buf).await.unwrap();
                self.mdel(keys).await;
                self.save().await;
                0u8.write(buf).await.unwrap();
            }
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
            Command::Keys => {
                self.keys().await.write(buf).await.unwrap();
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
impl Del for CorporationBlueprintCache {
    type Id = Id;

    async fn del(&self, id: Self::Id) {
        self
            .cache
            .write()
            .await
            .remove(&id);
    }
}

#[async_trait]
impl Get for CorporationBlueprintCache {
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
impl Set for CorporationBlueprintCache {
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
impl Key for CorporationBlueprintCache {
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
impl Save for CorporationBlueprintCache {
    type Typ = Typ;

    fn file(&self) -> &str {
        "./db/corporation_blueprint.cachem"
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
pub struct CorporationBlueprintEntry {
    #[cfg_attr(feature = "with_serde", serde(skip_deserializing, default))]
    pub id:                  Uuid,
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
    #[cfg_attr(
        feature = "with_serde",
        serde(skip_deserializing, default = "default_corporation_id")
    )]
    pub corp_id:             CorporationId,
    #[cfg_attr(
        feature = "with_serde",
        serde(skip_deserializing, skip_serializing, default = "default_character_id")
    )]
    pub char_id:             CharacterId,
}

#[cfg(feature = "with_serde")]
fn default_character_id() -> CharacterId {
    0u32.into()
}

#[cfg(feature = "with_serde")]
fn default_corporation_id() -> CorporationId {
    0u32.into()
}
