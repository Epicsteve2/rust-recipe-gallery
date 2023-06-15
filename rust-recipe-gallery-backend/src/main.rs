use crate::errors::AppError;
// use database::controller::*;
mod custom_json_extractor;
mod database;
mod errors;
mod models;

use crate::models::Recipe;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use custom_json_extractor::InputJson;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use hyper::StatusCode;
use serde::Deserialize;
use std::env;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
struct PostRecipe {
    #[validate(length(min = 2, message = "title must be at least 2 characters"))]
    title: String,
    // // TODO: Validate every ingreident to have at least 2 characters as well.
    // #[validate(length(min = 1, message = "must have at least 1 ingredient"))]
    // ingredients: Vec<String>,
}

// #[derive(Debug, Serialize, Clone, Insertable)]
// #[diesel(table_name = recipes)]
// pub struct Recipe {
//     id: Uuid,
//     title: String,
//     // ingredients: Vec<String>,
// }
pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or("7979".to_string());

    // TODO: global var???
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    let app = Router::new()
        .route("/api/recipe/new", post(new_recipe))
        .with_state(pool);

    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
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
    // TODO: find a way to use '?' for result.
    let result = database::controller::post_recipe(pool, recipe)
        .await
        .unwrap();
    Ok((StatusCode::CREATED, result))
}
