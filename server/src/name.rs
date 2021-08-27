use std::collections::HashMap;

use crate::error::EveServerError;

use cachem::ConnectionPool;
use caph_db::CacheName;
use caph_eve_data_wrapper::TypeId;

#[derive(Clone)]
pub struct NameService(ConnectionPool);

impl NameService {
    pub fn new(pool: ConnectionPool) -> Self {
        Self(pool)
    }

    pub async fn resolve_id(&self, tid: TypeId) -> Result<Option<String>, EveServerError> {
        self.0
            .acquire()
            .await?
            .get::<_, _, String>(CacheName::Name, tid)
            .await
            .map_err(Into::into)
    }

    pub async fn resolve_bulk(&self, ids: Vec<TypeId>) -> Result<Vec<String>, EveServerError> {
        let res = self.0
            .acquire()
            .await?
            .mget::<_, _, String>(CacheName::Name, ids)
            .await?
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();
        Ok(res)
    }

    pub async fn resolve_names_to_id_bulk(
        &self,
        names: Vec<String>
    ) -> Result<HashMap<u32, String>, EveServerError> {
        let mut names = names;
        names.sort();
        names.dedup();

        let mut pool = self.0
            .acquire()
            .await?;

        let keys = pool
            .keys::<_, u32>(CacheName::Name)
            .await?;
        let names = pool
            .mget::<_, _, String>(CacheName::Name, keys.clone())
            .await?
            .into_iter()
            .zip(keys.iter())
            .filter(|(n, _)| n.is_some())
            .map(|(n, k)| (n.unwrap(), k))
            .filter(|(n, _)| names.contains(n))
            .map(|(n, k)| (*k, n))
            .collect::<HashMap<_, _>>();
        dbg!(names.len());
        Ok(names)
    }
}
