use crate::CollectorError;

use axum::extract::Extension;
use axum::{AddExtensionLayer, Json, Router};
use axum::handler::get;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use sqlx::{Connection, PgPool};
use std::sync::{Arc, Mutex};

/// ENV variable for the address the server should bind to
const SERVER_BIND_ADDR: &str = "COLLECTOR_BIND_ADDR";

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
pub async fn start_server(
    pg:    PgPool,
    state: Arc<Mutex<TaskState>>
) -> Result<(), CollectorError> {
    let app = Router::new()
        .route("/health", get(health)).boxed()
        .route("/status", get(status)).boxed()
        .layer(AddExtensionLayer::new(pg))
        .layer(AddExtensionLayer::new(state))
        .into_make_service();

    let bind = std::env::var(SERVER_BIND_ADDR)
        .unwrap_or_else(|_| String::from("127.0.0.1:9090"))
        .parse()
        .map_err(|_| CollectorError::CouldNotParseServerListenAddr)?;
    axum::Server::bind(&bind)
        .serve(app)
        .await
        .map_err(|_| CollectorError::CouldNotStartServer)?;

    Ok(())
}

/// Gets a healtcheck of the service
///
/// # Returns
///
/// Either a status 200 or an error
///
async fn health(
    pg: Extension<PgPool>
) -> impl IntoResponse {
    let mut status = Health::set_postgres(HealthStatus::Ok);

    let con = pg.acquire().await;
    if let Ok(mut con) = con {
        if let Err(e) = con.ping().await {
            status = Health::set_postgres(
                HealthStatus::Error(e.to_string())
            );
        }
    } else if let Err(e) = con {
        status = Health::set_postgres(
            HealthStatus::Error(e.to_string())
        );
    }
    (StatusCode::OK, Json(status))
}

/// Gets the status of all tasks
///
/// # Returns
///
/// Status of all tasks
///
async fn status(
    state: Extension<Arc<Mutex<TaskState>>>
) -> Result<impl IntoResponse, CollectorError> {
    let state = { state.lock().unwrap().clone() };
    Ok((StatusCode::OK, Json(state)))
}

/// Health status of all services
#[derive(Clone, Debug, Serialize)]
pub struct Health {
    /// Status of the postgres
    postgres: HealthStatus
}

impl Health {
    /// Sets the status of the postgres
    ///
    /// # Params
    ///
    /// * `status` -> Status of the postgres
    ///
    /// # Returns
    ///
    /// New health instance
    ///
    pub fn set_postgres(status: HealthStatus) -> Self {
        Self {
            postgres: status
        }
    }
}

/// Represents a health status
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]

pub enum HealthStatus {
    /// The service is ok
    Ok,
    /// There was an error
    Error(String)
}

/// State of all tasks
#[derive(Clone, Debug, Default, Serialize)]
pub struct TaskState {
    /// Character task
    character: Option<Status>,
    /// SDE task
    sde:       Option<Status>,
}

impl TaskState {
    /// Sets the status of the character task
    ///
    /// # Params
    ///
    /// * `status` -> New status
    ///
    pub fn character_status(&mut self, status: Status) {
        self.character = Some(status);
    }

    /// Sets the status of the sde task
    ///
    /// # Params
    ///
    /// * `status` -> New status
    ///
    pub fn sde_status(&mut self, status: Status) {
        self.sde = Some(status);
    }
}

/// All states a service can have
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProcessStatus {
    /// The task currently runs
    Running,
    /// The task was successful
    Ok,
    /// The task had an error, see the error field for more information
    Error,
}

/// Represents the current status of a task
#[derive(Clone, Debug, Serialize)]
pub struct Status {
    /// Last timestamp the task was invoked
    last_ts:  i64,
    /// Formatted timestamp
    last_iso: String,

    /// Timestamp for the next task call
    next_ts:  i64,
    /// Formatted timestamp
    next_iso: String,

    /// Duration it took to run the task
    duration: u64,

    /// Current state of the task
    status:   ProcessStatus,
    /// Error, if there is any
    error:    Option<String>,
}

impl Status {
    /// Creates a new status instance
    ///
    /// # Params
    ///
    /// * `last_ts`  -> Timestamp of the last run
    /// * `last_iso` -> Timestamp as ISO format
    /// * `next_ts`  -> Timestamp of the next run
    /// * `next_iso` -> Timestamp as ISO format
    /// * `duration` -> Duration it took to execute the task
    /// * `status`   -> Result status
    /// * `error`    -> Error if there was any
    ///
    pub fn new(
        last_ts:  i64,
        last_iso: String,

        next_ts:  i64,
        next_iso: String,

        duration: u64,
        status:   ProcessStatus,

        error:     Option<String>
    ) -> Self {
        Self {
            last_ts,
            last_iso,

            next_ts,
            next_iso,

            duration,

            status,
            error
        }
    }
}
