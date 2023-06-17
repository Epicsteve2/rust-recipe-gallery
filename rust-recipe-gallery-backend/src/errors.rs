use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Json(#[from] axum::extract::rejection::JsonRejection),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    // I prolly don't need the following cuz I don't do anything special with the errors. But, it's cool to have.
    #[error(transparent)]
    DatabaseError(#[from] diesel::result::Error),
    #[error(transparent)]
    PoolError(#[from] bb8::RunError<diesel_async::pooled_connection::PoolError>),
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
    #[error("failed to read {direction:?} body: {body:?}")]
    BodyMiddleware { direction: String, body: String },
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // maybe this should be trace?
        tracing::debug!("{:#?}", &self);
        match &self {
            AppError::BodyMiddleware { .. } => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": self.to_string() // idk if this works or not lol
                })),
            ),
            AppError::Json(err) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": err.body_text()
                })),
            ),
            AppError::ValidationError(_) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "message": self.to_string()
                })),
            ),
            AppError::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": err.to_string()
                })),
            ),
            AppError::PoolError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": self.to_string()
                })),
            ),

            AppError::OtherError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": self.to_string()
                })),
            ),
        }
        .into_response()
    }
}

// pub fn send_response(status: StatusCode, message: &str) -> (StatusCode, Json<Value>) {
//     (
//         status,
//         Json(json!({
//             "message": message
//         })),
//     )
// }

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, Json<Value>)
where
    E: std::error::Error,
{
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "message": err.to_string()
        })),
    )
}

// ? operator doesn't work in some functions because of required for `Result<BufferRequestBody, AppError>` to implement `FromResidual<Result<Infallible, axum::extract::rejection::BytesRejection>>`
// idk why it happens, but this is the workaround
pub fn wrap_anyhow<E>(err: E) -> AppError
where
    E: Into<anyhow::Error>,
{
    AppError::OtherError(err.into())
}
