use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest},
    http::Request,
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

/// This extractor provides an expected JSON based API response
/// with the error that `axum::Json` could potentially return.
pub struct InputJson<T>(pub T);

// ----------------------------------------------------------------------------
// Manual implementation of `FromRequest` that wraps `axum::Json` extractor.
// ----------------------------------------------------------------------------
// Pros&Cons:
// + Powerful API: Implementing `FromRequest` grants access to `RequestParts`
//   and `async/await`. This means that you can create more powerful rejections
// - Boilerplate: Requires creating a new extractor for every custom rejection
// - Complexity: Manually implementing `FromRequest` results on more complex code
#[async_trait]
impl<S, B, T> FromRequest<S, B> for InputJson<T>
where
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    S: Send + Sync,
    B: Send + 'static,
{
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();

        let req = Request::from_parts(parts, body);

        match Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            // Convert the error from `axum::Json` into whatever we want.
            Err(rejection) => Err((
                rejection.status(),
                Json(json!({
                    "message": rejection.body_text(),
                })),
            )),
        }
    }
}
