use async_trait::*;
use cachem::{Parse, Cache, Command, Get2, Set2, Save};
use caph_eve_data_wrapper::{AllianceId, CharacterId, CorporationId};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::BufStream;
use tokio::net::TcpStream;
use tokio::sync::{RwLock, watch::Receiver};

type Id  = Uuid;
type Val = LoginEntry;
type Typ = HashMap<Id, Val>;

pub struct LoginCache {
    cache: RwLock<Typ>,
    cnc:   Receiver<Command>,
}

impl LoginCache {
    pub fn new(cnc: Receiver<Command>) -> Self {
        Self {
            cache: RwLock::default(),
            cnc,
        }
    }
}

impl Into<Arc<dyn Cache>> for LoginCache {
    fn into(self) -> Arc<dyn Cache> {
        Arc::new(self)
    }
}

#[async_trait]
impl Cache for LoginCache {
    fn name(&self) -> String {
        "logins".into()
    }

    async fn handle(&self, cmd: Command, buf: &mut BufStream<TcpStream>) {
        match cmd {
            Command::Get => {
                let key = Id::read(buf).await.unwrap();
                let val = self.get(key).await;
                val.write(buf).await.unwrap();
            }
            Command::Set => {
                let key = Id::read(buf).await.unwrap();
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
impl Get2<Uuid, LoginEntry> for LoginCache {
    async fn get(&self, id: Uuid) -> Option<LoginEntry> {
        self
            .cache
            .read()
            .await
            .get(&id)
            .cloned()
    }
}

#[async_trait]
impl Set2<Uuid, LoginEntry> for LoginCache {
    async fn set(&self, id: Uuid, val: LoginEntry) {
        self
            .cache
            .write()
            .await
            .insert(id, val);
    }
}

#[async_trait]
impl Save for LoginCache {
    type Typ = Typ;

    fn file(&self) -> &str {
        "./db/logins.cachem"
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
pub struct LoginEntry {
    pub alliance_id:   AllianceId,
    pub alliance_name: String,
    pub corp_id:       CorporationId,
    pub corp_name:     String,
    pub user_id:       CharacterId,
    pub user_name:     String,
    pub aliase:        Vec<LoginEntry>,
}

