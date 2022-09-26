use caph_connector::TypeId;

#[derive(Debug)]
pub enum StructureError {
    FetchStructures(sqlx::Error),
    RigNotFound(TypeId),
}

impl warp::reject::Reject for StructureError { }
