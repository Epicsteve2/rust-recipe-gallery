use axum::response::IntoResponse;
use axum::{
    body::{Body, Bytes},
    extract::Request,
    middleware::Next,
    response::Response,
};
use http_body_util::BodyExt;

use crate::errors::AppError;

pub async fn print_body(req: Request, next: Next) -> Result<impl IntoResponse, AppError> {
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

// async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, AppError>
async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, AppError>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = body
        .collect()
        .await
        .map(|collected| collected.to_bytes())
        .map_err(|err| AppError::BodyMiddleware {
            direction: direction.to_string(),
            body: err.to_string(),
        })?;

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::info!("{direction} body = {body:?}");
    }

    Ok(bytes)
    // // let bytes = hyper::body::to_bytes(body).await.map_err(wrap_anyhow)?;
    // let bytes = axum::body::to_bytes(body, usize::MAX)
    //     .await
    //     // not sure why the normal questionmark doesn't work...
    //     // Body is from http_body::Body
    //     // TODO: Possible change?
    //     .map_err(|err| AppError::BodyMiddleware {
    //         direction: direction.to_string(),
    //         body: err.to_string(),
    //     })?;

    // if let Ok(body) = std::str::from_utf8(&bytes) {
    //     tracing::info!("{} body = {:?}", direction, body);
    // }

    // Ok(bytes)
}
