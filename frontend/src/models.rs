use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Validate)]
pub struct PostRecipe {
    #[validate(length(min = 2, message = "must be at least 2 characters"))]
    pub title: String,
    #[validate(length(min = 2, message = "must have at least 1 ingredient"))]
    pub ingredients: String,
    #[validate(length(min = 2, message = "must have a body"))]
    pub body: String,
}

#[derive(Debug, serde::Deserialize, Clone, Serialize)] // need clone for signals
pub struct Recipe {
    pub id: Uuid,
    pub title: String,
    pub ingredients: String,
    pub body: String,
}

#[derive(Error, Debug)] // need clone for signals
pub enum AppError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    GlooError(#[from] gloo_net::Error),
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
}

#[derive(Debug, serde::Deserialize, Clone, Serialize)] // need clone for signals
pub struct RecipeCommentsJson {
    pub results: Vec<RecipeComment>,
}

#[derive(Debug, serde::Deserialize, Clone, Serialize)] // need clone for signals
pub struct RecipeComment {
    pub comment: String,
    pub id: Uuid,
    pub recipe_id: Uuid,
}
