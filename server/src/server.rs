mod asset;
mod auth;
mod character;
mod industry;
mod item;
mod universe;

use crate::{AssetService, IndustryService, ItemService, ProjectService, UniverseService};
use crate::auth_service::AuthService;
use crate::error::ServerError;
use crate::character::CharacterService;

use axum::{AddExtensionLayer, Router};
use caph_core::MarketService;
use sqlx::PgPool;

/// ENV variable for the address the server should bind to
const SERVER_BIND_ADDR: &str = "SERVER_BIND_ADDR";

pub async fn start(
    pool:              PgPool,
    asset_service:     AssetService,
    auth_service:      AuthService,
    character_service: CharacterService,
    industry_service:  IndustryService,
    item_service:      ItemService,
    project_service:   ProjectService,
    universe_service:  UniverseService,

    project_service2:  caph_core::ProjectService,
) -> Result<(), ServerError> {
    let app = Router::new()
        .nest("/api",
            Router::new()
                    .nest("/asset", asset::router())
                    .nest("/auth", auth::router())
                    .nest("/character", character::router())
                    .nest("/industry", industry::router())
                    .nest("/item", item::router())
                    .nest("/universe", universe::router())
        )
        .nest(
            "/api/v1",
            Router::new()
                    .nest("/auth", crate::auth::router())
                    .nest("/projects", crate::project::router())
        )
        .layer(AddExtensionLayer::new(asset_service))
        .layer(AddExtensionLayer::new(auth_service))
        .layer(AddExtensionLayer::new(character_service))
        .layer(AddExtensionLayer::new(industry_service))
        .layer(AddExtensionLayer::new(item_service))
        .layer(AddExtensionLayer::new(project_service))
        .layer(AddExtensionLayer::new(universe_service))
        .layer(AddExtensionLayer::new(project_service2))
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
