mod database;
mod errors;
mod middleware;
mod models;

use crate::models::{PatchRecipe, PostRecipe, Recipe};
use crate::{errors::AppError, middleware::print_body_middleware};

use axum::routing::delete;
use axum::{
    extract::State,
    http::{Method, StatusCode},
    middleware as auxm_middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use middleware::{custom_json_extractor::InputJson, custom_path_extractor::InputPath};
use models::{Comment, PatchComment, PostComment};
use serde_json::{json, Value};
use std::{env, net::SocketAddr};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;
use validator::Validate;

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // I don't really get this, but this is debug logs lol. set with `export RUST_LOG=DEBUG`
    // docs are really confusing...
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "rust_recipe_gallery_backend=debug,tower_http::trace=info,info".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    let port = std::env::var("RUST_RECIPE_GALLERY_BACKEND_PORT")
        .map(|port_string| port_string.parse::<u16>().unwrap())
        .unwrap_or(7979);
    let database_url = env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:123456@db/recipe-gallery".to_string());

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = bb8::Pool::builder().build(config).await?;

    let app = Router::new()
        .route("/api/recipe/new", post(post_recipe))
        .route("/api/recipe", get(get_all_recipe))
        .route(
            "/api/recipe/:recipe_id",
            get(get_recipe).patch(patch_recipe).delete(delete_recipe),
        )
        .route(
            "/api/recipe/:recipe_id/comments",
            get(get_comments).patch(patch_comment).post(post_comment),
        )
        .route(
            "/api/recipe/:recipe_id/comments/:comment_id",
            delete(delete_comment),
        )
        .layer(auxm_middleware::from_fn(print_body_middleware::print_body))
        .layer(
            CorsLayer::new()
                // .allow_origin("http://0.0.0.0:7979".parse::<HeaderValue>().unwrap())
                .allow_origin(tower_http::cors::Any)
                .allow_methods([Method::GET, Method::POST, Method::PATCH])
                .allow_headers([axum::http::header::CONTENT_TYPE]),
        )
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

    // idk how this works lol
    // let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on http://{addr}");
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
        ingredients: payload.ingredients,
        body: payload.body,
    };
    let result = database::controller::create_recipe(pool, recipe).await?;
    Ok((StatusCode::CREATED, Json(result)))
}

// Should return an object instead of an array, oops
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
    payload.validate()?;
    let result = database::controller::update_recipe(pool, recipe_id, payload).await?;
    Ok((StatusCode::OK, Json(result)))
}

async fn delete_recipe(
    State(pool): State<Pool>,
    InputPath(recipe_id): InputPath<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let result = database::controller::delete_recipe(pool, recipe_id).await?;
    Ok((StatusCode::OK, Json(result)))
}

async fn post_comment(
    State(pool): State<Pool>,
    InputJson(payload): InputJson<PostComment>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;
    let comment = Comment {
        id: Uuid::new_v4(),
        recipe_id: payload.recipe_id,
        comment: payload.comment,
    };
    let result = database::controller::create_comment(pool, comment).await?;
    Ok((StatusCode::CREATED, Json(result)))
}

async fn get_comments(State(pool): State<Pool>) -> Result<impl IntoResponse, AppError> {
    let result = database::controller::read_all_comments(pool).await?;
    Ok((
        StatusCode::CREATED,
        Json(json!({
            "results": result
        })),
    ))
}

async fn patch_comment(
    State(pool): State<Pool>,
    InputPath(comment_id): InputPath<Uuid>,
    InputJson(new_comment): InputJson<PatchComment>,
) -> Result<impl IntoResponse, AppError> {
    new_comment.validate()?;
    let result = database::controller::update_comment(pool, comment_id, new_comment).await?;
    Ok((StatusCode::OK, Json(result)))
}

async fn delete_comment(
    State(pool): State<Pool>,
    InputPath((_, comment_id)): InputPath<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    let result = database::controller::delete_comment(pool, comment_id).await?;
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
