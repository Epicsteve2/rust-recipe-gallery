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
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_recipe_gallery_backend=info".into()), // idk why, but dashes are replaced with underscores...
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
        // I don't think this is doing anything
        // EDIT: nvm I had to put this after lol
        // .layer(
        //     TraceLayer::new_for_http()
        //         .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        //         .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        // )
        .layer(middleware::from_fn(print_body_middleware::print_body))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let uri = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    tracing::info_span!(
                        "request",
                        method = ?request.method(),
                        uri,
                        version = ?request.version(),
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // ...
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        // ...
                    },
                ),
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
