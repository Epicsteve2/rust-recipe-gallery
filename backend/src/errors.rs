use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use diesel::result::{
    DatabaseErrorKind,
    Error::{DatabaseError, NotFound, QueryBuilderError},
};
use thiserror::Error;

use crate::to_response;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Json(#[from] axum::extract::rejection::JsonRejection),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    DatabaseError(#[from] diesel::result::Error),
    #[error("failed to read {direction:?} body: {body:?}")]
    BodyMiddleware { direction: String, body: String },
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // maybe this should be trace?
        tracing::debug!("{:#?}", &self);
        match &self {
            AppError::BodyMiddleware { .. } => {
                to_response(StatusCode::BAD_REQUEST, &self.to_string())
            }
            AppError::Json(err) => to_response(StatusCode::BAD_REQUEST, &err.body_text()),
            AppError::ValidationError(_) => {
                to_response(StatusCode::UNPROCESSABLE_ENTITY, &self.to_string())
            }
            AppError::DatabaseError(err) => match err {
                // TODO: i wish this could be better... somehow, embed what wasn't found
                NotFound => to_response(StatusCode::NOT_FOUND, &err.to_string()),
                // TODO: same
                QueryBuilderError(_) => to_response(StatusCode::OK, &self.to_string()),
                DatabaseError(DatabaseErrorKind::ForeignKeyViolation, _) => {
                    to_response(StatusCode::UNPROCESSABLE_ENTITY, &self.to_string())
                }
                _ => to_response(StatusCode::INTERNAL_SERVER_ERROR, &self.to_string()),
            },
            AppError::OtherError { .. } => {
                to_response(StatusCode::INTERNAL_SERVER_ERROR, &self.to_string())
            }
        }
        .into_response()
    }
}

// the '?' operator doesn't work in some functions
// idk why it happens, but this is the workaround
#[allow(dead_code)]
pub fn wrap_anyhow<E>(err: E) -> AppError
where
    E: Into<anyhow::Error>,
{
    AppError::OtherError(err.into())
}

// or use this
// more info: https://github.com/dtolnay/thiserror/issues/154
// and https://github.com/dtolnay/thiserror/issues/52
impl From<bb8::RunError<diesel_async::pooled_connection::PoolError>> for AppError {
    fn from(err: bb8::RunError<diesel_async::pooled_connection::PoolError>) -> Self {
        AppError::OtherError(err.into())
    }
}
