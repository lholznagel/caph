mod asset;
mod auth;
mod character;
mod item;

use crate::{AssetService, ItemService, ProjectService};
use crate::auth_service::AuthService;
use crate::error::ServerError;
use crate::character::CharacterService;

use axum::{AddExtensionLayer, Router};

/// ENV variable for the address the server should bind to
const SERVER_BIND_ADDR: &str = "SERVER_BIND_ADDR";

pub async fn start(
    asset_service:     AssetService,
    auth_service:      AuthService,
    character_service: CharacterService,
    item_service:      ItemService,
    project_service:   ProjectService,

    project_service2:  caph_core::ProjectService,
) -> Result<(), ServerError> {
    let app = Router::new()
        .nest("/api",
            Router::new()
                    .nest("/asset", asset::router())
                    .nest("/auth", auth::router())
                    .nest("/character", character::router())
                    .nest("/item", item::router())
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
        .layer(AddExtensionLayer::new(item_service))
        .layer(AddExtensionLayer::new(project_service))
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
