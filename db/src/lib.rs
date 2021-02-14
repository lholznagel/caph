mod blueprint;
mod id_name;
mod item_material;
mod item;
mod market_order;
mod market_order_info;
mod region;
mod station;

pub use self::blueprint::*;
pub use self::id_name::*;
pub use self::item_material::*;
pub use self::item::*;
pub use self::market_order::*;
pub use self::market_order_info::*;
pub use self::region::*;
pub use self::station::*;

use cachem::{CachemError, Parse};

#[derive(Debug, Default)]
pub struct EmptyResponse;

use tokio::io::{AsyncBufRead, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
#[async_trait::async_trait]
impl Parse for EmptyResponse {
    async fn read<B>(
        buf: &mut B,
    ) -> Result<Self, CachemError>
    where
        B: AsyncBufRead + AsyncRead + Send + Unpin  {

        let _ = buf.read_u8().await?;
        Ok(Self::default())
    }

    async fn write<B>(
        &self,
        buf: &mut B,
    ) -> Result<(), CachemError>
    where
        B: AsyncWrite + Send + Unpin {

        buf.write_u8(0u8).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum Actions {
    Fetch,
    Insert,
    Update,
    Delete,
    Lookup,
    Raw,
}

impl Into<u8> for Actions {
    fn into(self) -> u8 {
        match self {
            Self::Fetch  => 0u8,
            Self::Insert => 1u8,
            Self::Update => 2u8,
            Self::Delete => 3u8,
            Self::Lookup => 4u8,
            Self::Raw    => 5u8,
        }
    }
}

impl From<u8> for Actions {
    fn from(x: u8) -> Self {
        match x {
            0 => Actions::Fetch,
            1 => Actions::Insert,
            2 => Actions::Update,
            3 => Actions::Delete,
            4 => Actions::Lookup,
            5 => Actions::Raw,
            _ => panic!("Unrecognized actions {}", x),
        }
    }
}

#[derive(Debug)]
pub enum Caches {
    Blueprint,
    IdName,
    Item,
    ItemMaterial,
    MarketOrder,
    MarketOrderInfo,
    Region,
    Station,
}

impl Into<u8> for Caches {
    fn into(self) -> u8 {
        match self {
            Self::Blueprint          =>  0u8,
            Self::IdName             =>  1u8,
            Self::Item               =>  2u8,
            Self::ItemMaterial       =>  3u8,
            Self::MarketOrder        =>  7u8,
            Self::MarketOrderInfo    =>  8u8,
            Self::Region             =>  9u8,
            Self::Station            => 10u8,
        }
    }
}

impl From<u8> for Caches {
    fn from(x: u8) -> Self {
        match x {
            0  => Self::Blueprint,
            1  => Self::IdName,
            2  => Self::Item,
            3  => Self::ItemMaterial,
            7  => Self::MarketOrder,
            8  => Self::MarketOrderInfo,
            9  => Self::Region,
            10 => Self::Station,
            _ => panic!("Unrecognized cache type {}", x),
        }
    }
}

#[async_trait::async_trait]
pub trait Raw<T: Parse> {
    type Error;
    type Response;

    async fn raw(&self, input: T) -> Result<Self::Response, Self::Error>;
}
