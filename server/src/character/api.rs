use crate::AuthUser;
use crate::character::CharacterService;
use crate::error::Error;

use axum::{Json, Router};
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get};
use caph_connector::CharacterId;
use tracing::instrument;

pub struct CharacterApi;

impl CharacterApi {
    pub fn router() -> Router {
        Router::new()
            .route("/alts", get(Self::alts))
            .route("/blueprints", get(Self::blueprints))
            .nest("/:id", Router::new()
                .route("/", delete(Self::remove))
                .route("/info", get(Self::info))
                .route("/refresh", get(Self::refresh))
            )
    }

    /// Gets a list of all alts from the main character.
    /// 
    /// # Params
    /// 
    /// * `service` -> [CharacterService]
    /// * `user`    -> Instance of an authenticated user
    /// 
    /// # Errors
    /// 
    /// If the database access fails.
    /// 
    /// # Returns
    /// 
    /// List of all alts from the logged in main character.
    /// 
    #[instrument(err)]
    async fn alts(
        service: Extension<CharacterService>,
        user:    AuthUser
    ) -> Result<impl IntoResponse, Error> {
        let cid = user.character_id().await?;

        service
            .alts(cid)
            .await
            .map(|x| (StatusCode::OK, Json::from(x)))
            .map_err(Into::into)
    }

    /// Gets a list of all blueprints the character and its alts own
    /// 
    /// # Params
    /// 
    /// * `service` -> [CharacterService]
    /// * `user`    -> Instance of an authenticated user
    /// 
    /// # Errors
    /// 
    /// If the database access fails.
    /// 
    /// # Returns
    /// 
    /// List of all alts from the logged in main character.
    /// 
    #[instrument(err)]
    async fn blueprints(
        service: Extension<CharacterService>,
        user:    AuthUser
    ) -> Result<impl IntoResponse, Error> {
        let cid = user.character_id().await?;

        service
            .blueprints(cid)
            .await
            .map(|x| (StatusCode::OK, Json::from(x)))
            .map_err(Into::into)
    }

    /// Gets character, corporation and alliance information of the given
    /// [CharacterId].
    /// If the character is not stored in the database it will return 
    /// status code 404.
    /// 
    /// # Params
    /// 
    /// * `service` -> [CharacterService]
    /// * `cid`     -> [CharacterId] of the character the info is needed.
    /// 
    /// # Fails
    /// 
    /// If the database access fails.
    /// 
    /// # Returns
    /// 
    /// Information about the character.
    /// 
    #[instrument(err)]
    async fn info(
        service:   Extension<CharacterService>,
        Path(cid): Path<CharacterId>
    ) -> Result<impl IntoResponse, Error> {
        service
            .info(cid)
            .await
            .map(|x| (StatusCode::OK, Json(x)))
            .map_err(Into::into)
    }

    /// Removes the given user.
    /// If the user does not exist, nothing will happen.
    /// 
    /// # Params
    /// 
    /// `service` -> [CharacterService]
    /// `cid`     -> Id of the character that should be removed
    /// 
    /// # Errors
    /// 
    /// If the database access fails.
    /// 
    /// # Returns
    /// 
    /// Status code 204 - No content
    /// 
    #[instrument(err)]
    async fn remove(
        service:   Extension<CharacterService>,
        Path(cid): Path<CharacterId>
    ) -> Result<impl IntoResponse, Error> {
        service.remove(cid).await?;
        Ok((StatusCode::NO_CONTENT, Json("")))
    }

    /// Refreshs information about the given user.
    /// If the user does not exist, nothing will happen.
    /// 
    /// # Params
    /// 
    /// `service` -> [CharacterService]
    /// `cid`     -> Id of the character that should be refreshed
    /// 
    /// # Errors
    /// 
    /// If the database access fails.
    /// 
    /// # Returns
    /// 
    /// Status code 204 - No content
    /// 
    #[instrument(err)]
    async fn refresh(
        service:   Extension<CharacterService>,
        Path(cid): Path<CharacterId>
    ) -> Result<impl IntoResponse, Error> {
        service.refresh(cid).await?;
        Ok((StatusCode::OK, Json(())))
    }
}
