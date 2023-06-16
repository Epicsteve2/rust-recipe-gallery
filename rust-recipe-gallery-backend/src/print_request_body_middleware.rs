use axum::{
    async_trait,
    body::{self, BoxBody, Bytes, Full},
    extract::FromRequest,
    http::Request,
    middleware::Next,
    response::IntoResponse,
};

use crate::errors::{wrap_anyhow, AppError};

// middleware that shows how to consume the request body upfront
pub async fn print_request_body(
    request: Request<BoxBody>,
    next: Next<BoxBody>,
) -> Result<impl IntoResponse, AppError> {
    let request = buffer_request_body(request).await?;

    Ok(next.run(request).await)
}

// the trick is to take the request apart, buffer the body, do what you need to do, then put
// the request back together
async fn buffer_request_body(request: Request<BoxBody>) -> Result<Request<BoxBody>, AppError> {
    let (parts, body) = request.into_parts();

    // this wont work if the body is an long running stream
    let bytes = hyper::body::to_bytes(body).await.map_err(wrap_anyhow)?;

    log_request_body(bytes.clone());

    Ok(Request::from_parts(parts, body::boxed(Full::from(bytes))))
}

fn log_request_body(bytes: Bytes) {
    tracing::info!(body = ?bytes);
}

// extractor that shows how to consume the request body upfront
struct BufferRequestBody(Bytes);

// we must implement `FromRequest` (and not `FromRequestParts`) to consume the body
#[async_trait]
impl<S> FromRequest<S, BoxBody> for BufferRequestBody
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request<BoxBody>, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state).await.map_err(wrap_anyhow)?;

        log_request_body(body.clone());

        Ok(Self(body))
    }
}
