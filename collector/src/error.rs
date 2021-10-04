//! Thin error wrapper that is used in the application
use axum::body::{Bytes, Full};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use std::convert::Infallible;

/// All errors that can be thrown in this module
#[derive(Debug)]
pub enum CollectorError {
    /// While getting all character ids and refresh tokens
    SelectCharacterEntries(sqlx::Error),

    /// While deleting all old character assets and error occured
    DeletingCharacterAssets(sqlx::Error),
    /// Error while inserting character assets
    InsertingCharacterAssets(sqlx::Error),
    /// Error while fetching character assets
    CouldNotGetCharacterAssets(caph_connector::ConnectError),

    /// While deleting all old character asset names and error occured
    DeletingCharacterAssetNames(sqlx::Error),
    /// Error while inserting character asset names
    InsertingCharacterAssetNames(sqlx::Error),
    /// Error while fetching character asset names
    CouldNotGetCharacterAssetNames(caph_connector::ConnectError),
    /// Error while fetching character asset names
    CouldNotGetCharacterAssetItemIds(sqlx::Error),

    /// Error while inserting character blueprints
    InsertingCharacterBlueprints(sqlx::Error),
    /// Error while fetching character blueprints
    CouldNotGetCharacterBlueprints(caph_connector::ConnectError),

    /// Error while inserting SDE items
    InsertingSdeItem(sqlx::Error),
    /// Error while inserting SDE reprocess data
    InsertingSdeReprocess(sqlx::Error),
    /// Deleting the current SDE reporcessing data failed
    DeletingSdeReprocess(sqlx::Error),
    /// Error while inserting SDE blueprint data
    InsertingSdeBlueprint(sqlx::Error),
    /// Error while inserting SDE blueprint_material data
    InsertingSdeBlueprintMaterial(sqlx::Error),
    /// Error while inserting SDE blueprint_skill data
    InsertingSdeBlueprintSkill(sqlx::Error),
    /// Deleting the current SDE blueprint data failed
    DeletingSdeBlueprint(sqlx::Error),
    /// Error while inserting SDE schematic data
    InsertingSdeSchematic(sqlx::Error),
    /// Error while inserting SDE schematic_material data
    InsertingSdeSchematicMaterial(sqlx::Error),
    /// Deleting the current SDE schematic data failed
    DeletingSdeSchematic(sqlx::Error),
    /// Error while inserting SDE station data
    InsertingSdeStation(sqlx::Error),
    /// Deleting the current SDE station data failed
    DeletingSdeStation(sqlx::Error),
    /// Error loading a file from the zip file
    LoadSdeFile(caph_connector::ConnectError),
    /// Error while downloading the SDE zip file
    LoadingSdeZip(caph_connector::ConnectError),

    /// An eve client could not be constructed
    CouldNotCreateEveClient(caph_connector::ConnectError),
    /// A transaction could not be established
    TransactionBeginNotSuccessfull(sqlx::Error),
    /// The transactin could not be commited
    TransactionCommitNotSuccessfull(sqlx::Error),

    /// Error parsing the listener address
    CouldNotParseServerListenAddr,
    /// Server could not be started for some reason
    CouldNotStartServer
}
impl std::error::Error for CollectorError {}

impl std::fmt::Display for CollectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) 
    }
}

impl IntoResponse for CollectorError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        let (status, msg) = match self {
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        };

        let body = Json(json!({
            "error": msg
        }));

        (status, body).into_response()
    }
}
