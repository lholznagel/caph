use async_trait::*;
use cachem::{Parse, v2::{Cache, Command, Del, Get, Key, Set, Save}};
use caph_eve_data_wrapper::{CharacterId, ItemId, SolarSystemId, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::BufStream;
use tokio::net::TcpStream;
use tokio::sync::{RwLock, watch::Receiver};
use uuid::Uuid;

type Idx = Uuid;
type Val = ProjectEntry;
type Typ = HashMap<Idx, Val>;

#[derive(Clone)]
pub struct ProjectCache {
    cache: Arc<RwLock<Typ>>,
    cnc:   Receiver<Command>,
}

impl ProjectCache {
    pub fn new(cnc: Receiver<Command>) -> Self {
        Self {
            cache: Arc::new(RwLock::default()),
            cnc,
        }
    }
}

impl Into<Arc<Box<dyn Cache>>> for ProjectCache {
    fn into(self) -> Arc<Box<dyn Cache>> {
        Arc::new(Box::new(self))
    }
}

#[async_trait]
impl Cache for ProjectCache {
    fn name(&self) -> String {
        "projects".into()
    }

    async fn handle(&self, cmd: Command, buf: &mut BufStream<TcpStream>) {
        match cmd {
            Command::Del => {
                let key = Idx::read(buf).await.unwrap();
                self.del(key).await;
                self.save().await;
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
impl Del for ProjectCache {
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
impl Get for ProjectCache {
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
impl Set for ProjectCache {
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
impl Key for ProjectCache {
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
impl Save for ProjectCache {
    type Typ = Typ;

    fn file(&self) -> &str {
        "./db/projects.cachem"
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
pub struct ProjectEntry {
    pub id:         Uuid,
    pub name:       String,
    pub system:     SolarSystemId,
    pub chest:      ItemId,
    pub blueprints: Vec<ProjectBlueprintEntry>,
    pub user_id:    CharacterId,
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct ProjectBlueprintEntry {
    pub bpid: TypeId,
    pub runs: u32
}
