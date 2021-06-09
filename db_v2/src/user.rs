use async_trait::*;
use cachem::{Parse, v2::{Cache, Command, Get, Key, Set, Save}};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::BufStream;
use tokio::net::TcpStream;
use tokio::sync::{RwLock, watch::Receiver};

type Idx = u32;
type Val = UserEntry;
type Typ = HashMap<Idx, Val>;

pub struct UserCache {
    cache: RwLock<Typ>,
    cnc:   Receiver<Command>,
}

impl UserCache {
    pub fn new(cnc: Receiver<Command>) -> Self {
        Self {
            cache: RwLock::default(),
            cnc,
        }
    }
}

impl Into<Arc<Box<dyn Cache>>> for UserCache {
    fn into(self) -> Arc<Box<dyn Cache>> {
        Arc::new(Box::new(self))
    }
}

#[async_trait]
impl Cache for UserCache {
    fn name(&self) -> String {
        "users".into()
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
                0u8.write(buf).await.unwrap();
            }
            Command::Set => {
                let key = Idx::read(buf).await.unwrap();
                let val = Val::read(buf).await.unwrap();
                self.set(key, val).await;
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
impl Get for UserCache {
    type Idx   = Idx;
    type Res   = Val;
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
impl Set for UserCache {
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
impl Key for UserCache {
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
impl Save for UserCache {
    type Typ = Typ;

    fn file(&self) -> &str {
        "./db/users.cachem"
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
pub struct UserEntry {
    pub user_id:       u32,
    pub name:          String,
    pub aliase:        Vec<UserEntry>,
    pub access_token:  String,
    pub refresh_token: String,
    pub token:         String,
}

impl UserEntry {
    pub fn new(
        user_id:       u32,
        name:          String,
        access_token:  String,
        refresh_token: String,
        token:         String,
    ) -> Self {
        Self {
            user_id,
            name,
            aliase: Vec::new(),
            access_token,
            refresh_token,
            token,
        }
    }
}

