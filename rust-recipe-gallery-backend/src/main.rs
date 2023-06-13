use crate::errors::AppError;

mod custom_json_extractor;
mod database;
mod errors;

use axum::extract::Json;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use custom_json_extractor::InputJson;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
struct PostRecipe {
    #[validate(length(min = 2, message = "title must be at least 2 characters"))]
    title: String,
    // TODO: Validate every ingreident to have at least 2 characters as well.
    #[validate(length(min = 1, message = "must have at least 1 ingredient"))]
    ingredients: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
struct Recipe {
    id: Uuid,
    title: String,
    ingredients: Vec<String>,
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or("7979".to_string());
    let app = Router::new().route("/api/recipe/new", post(new_recipe));

    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn new_recipe(
    InputJson(payload): InputJson<PostRecipe>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;
    let recipe = Recipe {
        id: Uuid::new_v4(),
        title: payload.title,
        ingredients: payload.ingredients,
    };
    dbg!(&recipe);
    Ok((StatusCode::CREATED, Json(recipe)))
}
