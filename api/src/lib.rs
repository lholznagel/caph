mod alliance;
mod contracts;
mod dogma;
mod error;
mod eve_client;
mod market;
mod routes;
mod universe;

pub use self::alliance::*;
pub use self::contracts::*;
pub use self::dogma::*;
pub use self::error::*;
pub use self::eve_client::*;
pub use self::market::*;
pub use self::routes::*;
pub use self::universe::*;

#[macro_export]
macro_rules! fetch {
    ($name:ident, $path:expr, $result:ty) => {
        pub async fn $name(&self) -> crate::error::Result<$result> {
            self.fetch($path)
                .await?
                .json()
                .await
                .map_err(crate::error::EveApiError::ReqwestError)
        }
    };
    ($name:ident, $path:expr, $id:ty, $result:ty) => {
        pub async fn $name(&self, id: $id) -> crate::error::Result<Option<$result>> {
            self.fetch_single_by_id($path, id.0, None).await
        }
    };
    ($name:ident, $path:expr, $sub_path:expr, $id:ty, $result:ty) => {
        pub async fn $name(&self, id: $id) -> crate::error::Result<Option<$result>> {
            self.fetch_single_by_id($path, id.0, Some($sub_path)).await
        }
    };
    ($name:ident, $path:expr, $sub_path:expr, $id:ty, Vec, $result:ty) => {
        pub async fn $name(&self, id: $id) -> crate::error::Result<Option<Vec<$result>>> {
            self.fetch_by_id($path, id.0, Some($sub_path)).await
        }
    };
}

#[macro_export]
macro_rules! id {
    ($name:ident, $fn_name:ident, $uri:expr) => {
        #[derive(
            Copy,
            Clone,
            Debug,
            Eq,
            PartialEq,
            Hash,
            Ord,
            PartialOrd,
            serde::Deserialize,
            serde::Serialize,
        )]
        pub struct $name(pub u32);

        impl EveClient {
            pub async fn $fn_name(&self) -> crate::error::Result<Vec<$name>> {
                self.fetch_ids($uri).await
            }
        }

        impl Id for $name {
            fn id(&self) -> u32 {
                self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

#[macro_export]
macro_rules! conversion_model {
    ($name:ident, $id_type:ty) => {
        #[derive(Clone, Debug, serde::Deserialize)]
        pub struct $name {
            pub id: $id_type,
            pub name: String,
        }
    };
}
