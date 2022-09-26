use caph_connector::Structure;
use sqlx::PgPool;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};
use warp::filters::BoxedFilter;

use crate::{AuthCharacter, with_authorization};
use super::service::{StructureService, with_structure_service};

#[derive(Clone, Debug)]
pub struct StructureApi;

impl StructureApi {
    /// Filters that build up the api for this part of the application
    pub fn api(
        pool:      PgPool,
        base_path: BoxedFilter<()>,
    ) -> BoxedFilter<(impl Reply,)> {
        let base_path = base_path
            .clone()
            .and(warp::path!("structures" / ..))
            .and(with_authorization(pool.clone()))
            .and(with_structure_service(pool.clone()))
            .boxed();

        let get_all = base_path
            .clone()
            .and(warp::path::end())
            .and(warp::get())
            .and_then(Self::get_all)
            .boxed();

        let get_one = base_path
            .clone()
            .and(warp::path!(Uuid))
            .and(warp::get())
            .and_then(Self::get_one)
            .boxed();

        let add = base_path
            .clone()
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::add)
            .boxed();

        let update = base_path
            .clone()
            .and(warp::path!(Uuid))
            .and(warp::put())
            .and(warp::body::json())
            .and_then(Self::update)
            .boxed();

        let delete = base_path
            .clone()
            .and(warp::path!(Uuid))
            .and(warp::delete())
            .and_then(Self::delete)
            .boxed();

        get_all
            .or(get_one)
            .or(add)
            .or(update)
            .or(delete)
            .boxed()
    }

    async fn get_all(
        auth:    AuthCharacter,
        service: StructureService,
    ) -> Result<impl Reply, Rejection> {
        // TODO: check if the user is permitted
        Ok(warp::reply::json(&()))
    }

    async fn get_one(
        auth:    AuthCharacter,
        service: StructureService,
        sid:     Uuid,
    ) -> Result<impl Reply, Rejection> {
        // TODO: check if the user is permitted
        Ok(warp::reply::json(&()))
    }

    async fn add(
        auth:    AuthCharacter,
        service: StructureService,
        body:    Structure,
    ) -> Result<impl Reply, Rejection> {
        // TODO: check if the user is permitted
        Ok(warp::reply::json(&()))
    }

    async fn update(
        auth:    AuthCharacter,
        service: StructureService,
        sid:     Uuid,
        body:    Structure,
    ) -> Result<impl Reply, Rejection> {
        // TODO: check if the user is permitted
        Ok(warp::reply::json(&()))
    }

    async fn delete(
        auth:    AuthCharacter,
        service: StructureService,
        sid:     Uuid,
    ) -> Result<impl Reply, Rejection> {
        // TODO: check if the user is permitted
        Ok(warp::reply::json(&()))
    }
}
