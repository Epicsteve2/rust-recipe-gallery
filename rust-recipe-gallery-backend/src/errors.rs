use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
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
