use crate::errors::AppError;
// use database::controller::*;
mod custom_json_extractor;
mod database;
mod errors;
mod models;
mod print_request_body_middleware;

use crate::models::Recipe;
use axum::body::{self};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{extract::State, Json};
use axum::{middleware, Router};
use custom_json_extractor::InputJson;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
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
        .layer(
            ServiceBuilder::new()
                .map_request_body(body::boxed)
                .layer(middleware::from_fn(
                    print_request_body_middleware::print_request_body,
                )),
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
    dbg!(&recipe);

    let result = database::controller::post_recipe(pool, recipe).await?;
    Ok((StatusCode::CREATED, result))
}
