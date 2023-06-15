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
    ValidationError(#[from] validator::ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ValidationError(_) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "error": self.to_string()
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
            "error": err.to_string()
        })),
    )
}
