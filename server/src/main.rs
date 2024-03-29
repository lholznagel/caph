use axum::{Extension, Router};
use caph_server::*;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::{convert::Infallible, sync::Arc};
use tracing_subscriber::EnvFilter;
use tracing::Level;
use warp::{filters::BoxedFilter, Filter};

/// ENV variable for the database URL
const PG_ADDR: &str = "DATABASE_URL";
/// ENV variable for the address the server should bind to
const SERVER_BIND_ADDR: &str = "SERVER_BIND_ADDR";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    if std::env::var("GIT_HEAD_SHORT").is_ok() {
        tracing_subscriber::fmt().with_max_level(Level::WARN).init();
    } else {
        tracing_subscriber::fmt()
            .pretty()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    }

    let pg_addr = std::env::var(PG_ADDR).expect("Expected that a DATABASE_URL ENV is set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&pg_addr)
        .await?;
    sqlx::migrate!().run(&pool).await?;

    let auth_service = AuthService::new(pool.clone());
    let character_service = CharacterService::new(pool.clone(), auth_service.clone());
    let item_service = ItemService::new(pool.clone());

    //let project_blueprint_service =
    //    ProjectBlueprintService::new(pool.clone(), character_service.clone());
    //let project_storage_service = ProjectStorageService::new(pool.clone());
    //let project_service = ProjectService::new(pool.clone(), project_blueprint_service.clone());

    let app = Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .nest("/auth", crate::AuthApi::router())
                // TODO: remove
                .nest("/character", crate::CharacterApi::router())
                .nest("/characters", crate::CharacterApi::router())
                .nest("/items", crate::ItemApi::router())
                .nest("/projects", crate::ProjectApi::router()),
        )
        .layer(Extension(auth_service.clone()))
        .layer(Extension(character_service.clone()))
        .layer(Extension(item_service))
        //.layer(Extension(project_service))
        //.layer(Extension(project_blueprint_service))
        //.layer(Extension(project_storage_service))
        .layer(Extension(pool.clone()))
        .into_make_service();

    let bind = std::env::var(SERVER_BIND_ADDR)
        .unwrap_or_else(|_| String::from("127.0.0.1:8080"))
        .parse()
        .map_err(|_| Error::CouldNotParseServerListenAddr)?;
    tracing::info!("Starting server");

    let server = Server::new(auth_service, pool);
    let v1 = axum::Server::bind(&bind).serve(app);

    let _ = tokio::join! {
        v1,
        server.listen(),
    };

    Ok(())
}

struct Server {
    auth: AuthService,
    pool: PgPool,
}

impl Server {
    pub fn new(auth: AuthService, pool: PgPool) -> Self {
        Self { auth, pool }
    }

    pub async fn listen(self) {
        let base_path = warp::path!("api" / "v2" / ..).boxed();

        let indy = IndustryApi::api(self.pool.clone(), base_path.clone());
        let projects = ProjectApiV2::api(self.pool.clone(), base_path.clone());
        let structure = StructureApi::api(self.pool.clone(), base_path.clone());

        let routes = indy
            .or(projects)
            .or(structure);
        warp::serve(routes).run(([127, 0, 0, 1], 10102)).await
    }
}
