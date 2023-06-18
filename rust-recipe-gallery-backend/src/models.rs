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
    // // this is dumb. might need another table for ingredients??? Doesn't seem possible to be double not null...
    // pub ingredients: Vec<Option<String>>,
}

#[derive(Debug, Deserialize, Validate, AsChangeset)]
#[diesel(table_name = crate::database::schema::recipes)]
pub struct PatchRecipe {
    // #[validate(required)]
    // TODO: validate
    pub title: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PostRecipe {
    #[validate(length(min = 2, message = "must be at least 2 characters"))]
    pub title: String,
    // // TODO: Validate every ingreident to have at least 2 characters as well.
    // #[validate(length(min = 1, message = "must have at least 1 ingredient"))]
    // ingredients: Vec<String>,
}
