#[derive(Debug)]
pub enum IndustryError {
    DeleteAssetsError(sqlx::Error),
    FetchCharacterLocationIds(sqlx::Error),
    FetchCharacterItemIds(sqlx::Error),
    FetchCharacterAssetName(caph_connector::ConnectError),
    SaveCharacterAssets(sqlx::Error),
    InsertCharacterAssetLocations(sqlx::Error),
    InsertCharacterAssetNames(sqlx::Error),
}

impl warp::reject::Reject for IndustryError { }
