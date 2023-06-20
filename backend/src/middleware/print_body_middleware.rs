use axum::response::IntoResponse;
use axum::{
    body::{Body, Bytes},
    http::Request,
    middleware::Next,
    response::Response,
};

use crate::errors::AppError;

pub async fn print_body(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, AppError> {
    tracing::debug!("{:#?}", req);
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));
    tracing::debug!("{:#?}", res);

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, AppError>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    // let bytes = hyper::body::to_bytes(body).await.map_err(wrap_anyhow)?;
    let bytes = hyper::body::to_bytes(body)
        .await
        // not sure why the normal questionmark doesn't work...
        // Body is from http_body::Body
        // TODO: Possible change?
        .map_err(|err| AppError::BodyMiddleware {
            direction: direction.to_string(),
            body: err.to_string(),
        })?;

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::info!("{} body = {:?}", direction, body);
    }

    Ok(bytes)
}
