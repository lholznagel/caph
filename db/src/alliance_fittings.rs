use async_trait::*;
use caph_eve_data_wrapper::TypeId;
use cachem::{Cache, Command, Del, Get2, Key, Parse, Save, Set};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::BufStream;
use tokio::net::TcpStream;
use tokio::sync::{RwLock, watch::Receiver};

type Id  = Uuid;
type Val = AllianceFittingEntry;
type Typ = HashMap<Id, Val>;

#[derive(Clone)]
pub struct AllianceFittingCache {
    cache: Arc<RwLock<Typ>>,
    cnc:   Receiver<Command>,
}

impl AllianceFittingCache {
    pub fn new(cnc: Receiver<Command>) -> Self {
        Self {
            cache: Arc::new(RwLock::default()),
            cnc,
        }
    }
}

impl Into<Arc<dyn Cache>> for AllianceFittingCache {
    fn into(self) -> Arc<dyn Cache> {
        Arc::new(self)
    }
}

#[async_trait]
impl Cache for AllianceFittingCache {
    fn name(&self) -> String {
        "alliance_fittings".into()
    }

    async fn handle(&self, cmd: Command, buf: &mut BufStream<TcpStream>) {
        match cmd {
            Command::Del => {
                let key = Id::read(buf).await.unwrap();
                self.del(key).await;
                self.save().await;
                0u8.write(buf).await.unwrap();
            }
            Command::MDel => {
                let keys = Vec::<Id>::read(buf).await.unwrap();
                self.mdel(keys).await;
                self.save().await;
                0u8.write(buf).await.unwrap();
            }
            Command::Get => {
                let key = Id::read(buf).await.unwrap();
                let val = self.get(key).await;
                val.write(buf).await.unwrap();
            }
            Command::MGet => {
                let keys = Vec::<Id>::read(buf).await.unwrap();
                let vals = self.mget(keys).await;
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
impl Del for AllianceFittingCache {
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
impl Get2<Id, AllianceFittingEntry> for AllianceFittingCache {
    async fn get(&self, id: Uuid) -> Option<AllianceFittingEntry> {
        self
            .cache
            .read()
            .await
            .get(&id)
            .cloned()
    }
}

#[async_trait]
impl Set for AllianceFittingCache {
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
impl Key for AllianceFittingCache {
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
impl Save for AllianceFittingCache {
    type Typ = Typ;

    fn file(&self) -> &str {
        "./db/alliance_fittings.cachem"
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
pub struct AllianceFittingEntry {
    pub fittings:   Vec<Fitting>,
    pub id:         Uuid,
    pub name:       String,
    pub url:        String,

    pub how_to_fit: Option<String>,
    pub how_to_fly: Option<String>,
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct Fitting {
    pub name:     String,
    pub type_ids: Vec<TypeId>
}

