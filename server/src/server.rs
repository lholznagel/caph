mod asset;
mod auth;
mod character;
mod industry;
mod item;
mod project;
mod universe;

use crate::{ItemService, ProjectService, UniverseService, AssetService, IndustryService};
use crate::character::CharacterService;
use crate::error::ServerError;
use crate::eve::EveService;

use axum::{AddExtensionLayer, Router};

/// ENV variable for the address the server should bind to
const SERVER_BIND_ADDR: &str = "SERVER_BIND_ADDR";

/// Starts the API.
///
/// WARNING: This function call is blocking.
///
/// # Params
///
/// * `asset_service`     -> Service for handling assets
/// * `character_service` -> Service for handling eve characters
/// * `eve_service`       -> Service for managing EVE authentication
///
/// # Errors
///
/// Fails if the server bind addr is invalid or if starting the server fails.
///
/// # Returns
///
/// Nothing
///
pub async fn start(
    asset_service:     AssetService,
    character_service: CharacterService,
    eve_service:       EveService,
    industry_service:  IndustryService,
    item_service:      ItemService,
    project_service:   ProjectService,
    universe_service:  UniverseService,
) -> Result<(), ServerError> {
    let app = Router::new()
        .nest("/api",
            Router::new()
                    .nest("/asset", asset::router())
                    .nest("/auth", auth::router())
                    .nest("/character", character::router())
                    .nest("/industry", industry::router())
                    .nest("/item", item::router())
                    .nest("/project", project::router())
                    .nest("/universe", universe::router())
        )
        .layer(AddExtensionLayer::new(asset_service))
        .layer(AddExtensionLayer::new(character_service))
        .layer(AddExtensionLayer::new(eve_service))
        .layer(AddExtensionLayer::new(industry_service))
        .layer(AddExtensionLayer::new(item_service))
        .layer(AddExtensionLayer::new(project_service))
        .layer(AddExtensionLayer::new(universe_service))
        .into_make_service();

    let bind = std::env::var(SERVER_BIND_ADDR)
        .unwrap_or_else(|_| String::from("127.0.0.1:8080"))
        .parse()
        .map_err(|_| ServerError::CouldNotParseServerListenAddr)?;
    axum::Server::bind(&bind)
        .serve(app)
        .await
        .map_err(|_| ServerError::CouldNotStartServer)?;

    Ok(())
}
