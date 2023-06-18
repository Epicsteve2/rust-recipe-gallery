use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, serde::Serialize, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = crate::database::schema::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    pub id: Uuid,
    pub title: String,
    pub ingredients: String,
    pub body: String,
}

#[derive(Debug, serde::Serialize, Queryable, Selectable, Insertable, Associations)]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = crate::database::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub comment: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PostComment {
    pub recipe_id: Uuid,
    #[validate(length(min = 1, message = "must have a body"))]
    pub comment: String,
}

#[derive(Debug, Deserialize, Validate, AsChangeset)]
#[diesel(table_name = crate::database::schema::comments)]
pub struct PatchComment {
    pub id: Uuid,
    #[validate(length(min = 1, message = "must have a body"))]
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Validate, AsChangeset)]
#[diesel(table_name = crate::database::schema::recipes)]
pub struct PatchRecipe {
    #[validate(length(min = 2, message = "must be at least 2 characters"))]
    pub title: Option<String>,
    #[validate(length(min = 2, message = "must have at least 1 ingredient"))]
    pub ingredients: Option<String>,
    #[validate(length(min = 1, message = "must have a body"))]
    pub body: Option<String>,
}

// mayube both should be same thing??
#[derive(Debug, Deserialize, Validate)]
pub struct PostRecipe {
    #[validate(length(min = 2, message = "must be at least 2 characters"))]
    pub title: String,
    #[validate(length(min = 2, message = "must have at least 1 ingredient"))]
    pub ingredients: String,
    #[validate(length(min = 2, message = "must have a body"))]
    pub body: String,
}
