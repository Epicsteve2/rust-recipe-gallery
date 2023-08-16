use gloo_net::http::Request;
use validator::Validate;

use crate::models::{AppError, PostRecipe, Recipe, RecipeComment, RecipeCommentsJson};

pub async fn post_recipe(
    title: String,
    ingredients: String,
    steps: String,
) -> Result<Recipe, AppError> {
    let recipe = PostRecipe {
        title,
        ingredients,
        body: steps,
    };
    recipe.validate()?;
    let json_response = Request::post("http://0.0.0.0:7979/api/recipe/new")
        .json(&recipe)?
        .send()
        .await?
        .json::<Recipe>()
        .await?;
    // not needed, cuz server never sending a post request... I think
    // let json_response = reqwest::Client::new()
    //     .post("http://0.0.0.0:7979/api/recipe/new")
    //     .json(&recipe)
    //     .send()
    //     .await?
    //     .json::<Recipe>()
    //     .await?;
    Ok(json_response)
}

#[cfg(not(feature = "ssr"))]
pub async fn get_all_recipes() -> Result<Vec<Recipe>, String> {
    let json_response = Request::get("http://0.0.0.0:7979/api/recipe")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<Recipe>>()
        .await
        .map_err(|e| e.to_string())?;
    Ok(json_response)
}

// Docs  say to do this to fix an error where the server is attempting to call WASM. wahtever, just use reqwest for both
#[cfg(feature = "ssr")]
pub async fn get_all_recipes() -> Result<Vec<Recipe>, String> {
    let json_response = reqwest::get("http://0.0.0.0:7979/api/recipe")
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<Recipe>>()
        .await
        .map_err(|e| e.to_string())?;
    Ok(json_response)
}

pub async fn get_recipe_by_id(id: String) -> Result<Recipe, String> {
    let json_response = reqwest::get(format!("http://0.0.0.0:7979/api/recipe/{id}").as_str())
        .await
        .map_err(|e| e.to_string())?
        .json::<Recipe>()
        .await
        .map_err(|e| e.to_string())?;
    Ok(json_response)
}

pub async fn patch_recipe_by_id(
    id: String,
    title: String,
    ingredients: String,
    steps: String,
) -> Result<Recipe, AppError> {
    let uuid_id = uuid::Uuid::try_parse(&id)?;
    let json_response = Request::patch(format!("http://0.0.0.0:7979/api/recipe/{id}").as_str())
        .json(&Recipe {
            id: uuid_id,
            title,
            ingredients,
            body: steps,
        })?
        .send()
        .await?
        .json::<Recipe>()
        .await?;
    Ok(json_response)
}

pub async fn get_comments_by_recipe_id(id: String) -> Result<Vec<RecipeComment>, String> {
    let json_response =
        reqwest::get(format!("http://0.0.0.0:7979/api/recipe/{id}/comments").as_str())
            .await
            .map_err(|e| e.to_string())?
            .json::<RecipeCommentsJson>()
            .await
            .map_err(|e| e.to_string())?;
    Ok(json_response.results)
}

pub async fn delete_comment_by_id(
    recipe_id: String,
    comment_id: String,
) -> Result<RecipeComment, String> {
    let json_response = reqwest::get(
        format!("http://0.0.0.0:7979/api/recipe/{recipe_id}/comments/{comment_id}").as_str(),
    )
    .await
    .map_err(|e| e.to_string())?
    .json::<RecipeComment>()
    .await
    .map_err(|e| e.to_string())?;
    Ok(json_response)
}

pub async fn delete_recipe_by_id(recipe_id: String) -> Result<Recipe, String> {
    let json_response =
        Request::delete(format!("http://0.0.0.0:7979/api/recipe/{recipe_id}").as_str())
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<Recipe>()
            .await
            .map_err(|e| e.to_string())?;
    Ok(json_response)
}
