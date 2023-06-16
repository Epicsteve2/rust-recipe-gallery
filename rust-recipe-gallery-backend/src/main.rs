mod custom_json_extractor;
mod database;
mod errors;
mod models;
mod print_request_body_middleware;

use crate::errors::AppError;
use crate::models::Recipe;

use axum::{
    body::Bytes,
    extract::{MatchedPath, State},
    http::{HeaderMap, Request, StatusCode},
    middleware,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
mod print_body_middleware;
use custom_json_extractor::InputJson;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use serde::Deserialize;
use serde_json::json;
use std::{env, net::SocketAddr, time::Duration};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
struct PostRecipe {
    #[validate(length(min = 2, message = "must be at least 2 characters"))]
    title: String,
    // // TODO: Validate every ingreident to have at least 2 characters as well.
    // #[validate(length(min = 1, message = "must have at least 1 ingredient"))]
    // ingredients: Vec<String>,
}

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[tokio::main]
async fn main() {
    // I don't really get this, but this is debug logs lol. set with `export RUST_LOG=DEBUG`
    // docs are really confusing...
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "rust_recipe_gallery_backend=info,tower_http::trace=info".into()
            }), // idk why, but dashes are replaced with underscores...
        )
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    let port = std::env::var("RUST_RECIPE_GALLERY_BACKEND_PORT")
        .unwrap_or("7979".to_string())
        .parse::<u16>()
        .unwrap();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or("postgres://rust-recipe-gallery:123456@db/recipe-gallery".to_string());

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    let app = Router::new()
        .route("/api/recipe/new", post(new_recipe))
        // this seems a lot more of a pain...
        // .layer(
        //     ServiceBuilder::new()
        //         .map_request_body(body::boxed)
        //         .layer(middleware::from_fn(
        //             print_request_body_middleware::print_request_body,
        //         )),
        // )
        .layer(middleware::from_fn(print_body_middleware::print_body))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::INFO), // can also .include_headers(true)
                )
                .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                // .make_span_with(tracing::debug_span!(
                //     "full request",
                //     request = tracing::field::Empty
                // ))
                // .on_request(|_request: &Request<_>, _span: &Span| {
                //     // tracing::debug_span!("full request", "{:?}", &_request);
                //     // tracing::debug!("{:?}", &_request);
                //     // _span.request = &_request;
                //     _span.record("request", format!("{:?}", &_request));
                // })
                .on_response(
                    tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO),
                )
                .on_body_chunk(tower_http::trace::DefaultOnBodyChunk::new())
                .on_eos(tower_http::trace::DefaultOnEos::new().level(tracing::Level::INFO))
                .on_failure(tower_http::trace::DefaultOnFailure::new().level(tracing::Level::INFO)),
        )
        .with_state(pool)
        .fallback(handler_404);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "message": "URL does not exist"
        })),
    )
}

async fn new_recipe(
    State(pool): State<Pool>,
    InputJson(payload): InputJson<PostRecipe>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;
    let recipe = Recipe {
        id: Uuid::new_v4(),
        title: payload.title,
        // ingredients: payload.ingredients,
    };
    // tracing::info!("{:?}", &recipe);

    let result = database::controller::post_recipe(pool, recipe).await?;
    Ok((StatusCode::CREATED, result))
}
