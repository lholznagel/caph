use async_trait::*;
use caph_eve_data_wrapper::TypeId;
use cachem::{Cache, Command, Get2, Key, Parse, Save, Set};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::BufStream;
use tokio::net::TcpStream;
use tokio::sync::{RwLock, watch::Receiver};

type Id  = TypeId;
type Val = ItemDogmaEntry;
type Typ = HashMap<Id, Val>;

pub struct ItemDogmaCache {
    cache: RwLock<Typ>,
    cnc:   Receiver<Command>,
}

impl ItemDogmaCache {
    pub fn new(cnc: Receiver<Command>) -> Self {
        Self {
            cache: RwLock::default(),
            cnc,
        }
    }
}

impl Into<Arc<dyn Cache>> for ItemDogmaCache {
    fn into(self) -> Arc<dyn Cache> {
        Arc::new(self)
    }
}

#[async_trait]
impl Cache for ItemDogmaCache {
    fn name(&self) -> String {
        "item_dogmas".into()
    }

    async fn handle(&self, cmd: Command, buf: &mut BufStream<TcpStream>) {
        match cmd {
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
impl Get2<TypeId, ItemDogmaEntry> for ItemDogmaCache {
    async fn get(&self, id: TypeId) -> Option<ItemDogmaEntry> {
        self
            .cache
            .read()
            .await
            .get(&id)
            .cloned()
    }
}

#[async_trait]
impl Set for ItemDogmaCache {
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
impl Key for ItemDogmaCache {
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
impl Save for ItemDogmaCache {
    type Typ = Typ;

    fn file(&self) -> &str {
        "./db/item_dogmas.cachem"
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
pub struct ItemDogmaEntry {
    pub attributes: Vec<DogmaAttribute>,
    pub effects:    Vec<DogmaEffect>
}

impl ItemDogmaEntry {
    pub fn new(
        attributes: Vec<DogmaAttribute>,
        effects:    Vec<DogmaEffect>
    ) -> Self {
        Self {
            attributes,
            effects
        }
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct DogmaAttribute {
    pub attr_id: TypeId,
    pub value:   f32,
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct DogmaEffect {
    pub eff_id:  TypeId,
    pub default: bool,
}
