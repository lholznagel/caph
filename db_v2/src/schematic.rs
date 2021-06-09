use async_trait::*;
use caph_eve_data_wrapper::{PlanetSchematicEntry, TypeId};
use cachem::{Parse, v2::{Cache, Command, Get, Key, Set, Save}};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::BufStream;
use tokio::net::TcpStream;
use tokio::sync::{RwLock, watch::Receiver};

type Idx = TypeId;
type Val = SchematicEntry;
type Typ = HashMap<Idx, Val>;

pub struct SchematicCache {
    cache: RwLock<Typ>,
    cnc:   Receiver<Command>,
}

impl SchematicCache {
    pub fn new(cnc: Receiver<Command>) -> Self {
        Self {
            cache: RwLock::default(),
            cnc,
        }
    }
}

impl Into<Arc<Box<dyn Cache>>> for SchematicCache {
    fn into(self) -> Arc<Box<dyn Cache>> {
        Arc::new(Box::new(self))
    }
}

#[async_trait]
impl Cache for SchematicCache {
    fn name(&self) -> String {
        "schematics".into()
    }

    async fn handle(&self, cmd: Command, buf: &mut BufStream<TcpStream>) {
        match cmd {
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
impl Get for SchematicCache {
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
impl Set for SchematicCache {
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
impl Key for SchematicCache {
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
impl Save for SchematicCache {
    type Typ = Typ;

    fn file(&self) -> &str {
        "./db/schematics.cachem"
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
pub struct SchematicEntry {
    pub sid:        TypeId,
    pub cycle_time: u32,
    pub name:       String,
    pub pins:       Vec<TypeId>,
    pub inputs:     Vec<MaterialSchematic>,
    pub output:     MaterialSchematic,
}

impl SchematicEntry {
    pub fn new(
        sid:        TypeId,
        cycle_time: u32,
        name:       String,
        pins:       Vec<TypeId>,
        inputs:     Vec<MaterialSchematic>,
        output:     MaterialSchematic,
    ) -> Self {
        Self {
            sid,
            cycle_time,
            name,
            pins,
            inputs,
            output,
        }
    }
}

impl From<&PlanetSchematicEntry> for SchematicEntry {
    fn from(x: &PlanetSchematicEntry) -> Self {
        let inputs = x
            .types
            .iter()
            .filter(|(_, y)| y.is_input)
            .map(|(pid, e)| MaterialSchematic { pid: *pid, quantity: e.quantity })
            .collect::<Vec<_>>();
        let output = x
            .types
            .iter()
            .find(|(_, y)| !y.is_input)
            .map(|(pid, e)| MaterialSchematic { pid: *pid, quantity: e.quantity })
            .unwrap();

        Self {
            sid: output.pid,
            cycle_time: x.cycle_time,
            name: x.name.get("en").unwrap_or(&String::new()).clone(),
            pins: x.pins.clone(),
            inputs,
            output
        }
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct MaterialSchematic {
    pub pid:      TypeId,
    pub quantity: u32,
}

impl MaterialSchematic {
    pub fn new(
        pid:      TypeId,
        quantity: u32,
    ) -> Self {
        Self {
            pid,
            quantity,
        }
    }
}

