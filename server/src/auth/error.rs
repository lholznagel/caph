#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    InvalidUser,

    CreateAuthClient(caph_connector::ConnectError),
    SqlError(sqlx::Error),
}

impl warp::reject::Reject for AuthError {}

impl From<sqlx::Error> for AuthError {
    fn from(x: sqlx::Error) -> Self {
        Self::SqlError(x)
    }
}
