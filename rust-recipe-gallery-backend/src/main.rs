mod custom_json_extractor;
mod custom_path_extractor;
mod database;
mod errors;
mod models;
mod print_body_middleware;

use crate::errors::AppError;
use crate::models::Recipe;

use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use custom_json_extractor::InputJson;
use custom_path_extractor::InputPath;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{env, net::SocketAddr};
use tower_http::trace::TraceLayer;
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

#[derive(Debug, Deserialize, Validate)]
pub struct PatchRecipe {
    // #[validate(required)]
    // TODO: validate
    title: Option<String>,
}

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // I don't really get this, but this is debug logs lol. set with `export RUST_LOG=DEBUG`
    // docs are really confusing...
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "rust_recipe_gallery_backend=info,tower_http::trace=info".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    let port = std::env::var("RUST_RECIPE_GALLERY_BACKEND_PORT")
        .map(|port_string| port_string.parse::<u16>().unwrap())
        .unwrap_or(7979);
    let database_url = env::var("DATABASE_URL")
        .unwrap_or("postgres://rust-recipe-gallery:123456@db/recipe-gallery".to_string());

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = bb8::Pool::builder().build(config).await?;

    let app = Router::new()
        .route("/api/recipe/new", post(post_recipe))
        .route("/api/recipe", get(get_all_recipe))
        .route(
            "/api/recipe/:recipe_id",
            get(get_recipe).patch(patch_recipe),
        )
        .layer(middleware::from_fn(print_body_middleware::print_body))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::INFO),
                )
                .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(
                    tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO),
                )
                .on_body_chunk(tower_http::trace::DefaultOnBodyChunk::new())
                .on_eos(tower_http::trace::DefaultOnEos::new().level(tracing::Level::INFO))
                .on_failure(tower_http::trace::DefaultOnFailure::new().level(tracing::Level::INFO)),
        )
        .with_state(pool)
        .fallback(|| async { to_response(StatusCode::NOT_FOUND, "endpoint does not exist") });

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn post_recipe(
    State(pool): State<Pool>,
    InputJson(payload): InputJson<PostRecipe>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;
    let recipe = Recipe {
        id: Uuid::new_v4(),
        title: payload.title,
        // ingredients: payload.ingredients,
    };
    let result = database::controller::create_recipe(pool, recipe).await?;
    Ok((StatusCode::CREATED, Json(result)))
}

async fn get_all_recipe(State(pool): State<Pool>) -> Result<impl IntoResponse, AppError> {
    let result = database::controller::read_all_recipe(pool).await?;
    Ok((StatusCode::OK, Json(result)))
}

async fn get_recipe(
    State(pool): State<Pool>,
    InputPath(recipe_id): InputPath<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let result = database::controller::read_one_recipe(pool, recipe_id).await?;
    Ok((StatusCode::OK, Json(result)))
}

async fn patch_recipe(
    State(pool): State<Pool>,
    InputPath(recipe_id): InputPath<Uuid>,
    InputJson(payload): InputJson<PatchRecipe>,
) -> Result<impl IntoResponse, AppError> {
    let result = database::controller::update_recipe(pool, recipe_id, payload).await?;
    Ok((StatusCode::OK, Json(result)))
}

pub fn to_response(status: StatusCode, message: &str) -> (StatusCode, Json<Value>) {
    (
        status,
        Json(json!({
            "message": message
        })),
    )
}
