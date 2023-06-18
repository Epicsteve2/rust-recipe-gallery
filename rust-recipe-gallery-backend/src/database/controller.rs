use crate::errors::AppError;
use crate::models::{Comment, PatchComment};
use crate::Recipe;
use crate::{PatchRecipe, Pool};

use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use uuid::Uuid;

pub async fn create_recipe(pool: Pool, recipe: Recipe) -> Result<Recipe, AppError> {
    let mut conn = pool.get().await?;
    use super::schema::recipes;

    let result = diesel::insert_into(recipes::table)
        .values(recipe)
        .returning(Recipe::as_returning())
        .get_result(&mut conn)
        .await?;

    Ok(result)
}

pub async fn read_all_recipe(pool: Pool) -> Result<Vec<Recipe>, AppError> {
    let mut conn = &mut pool.get().await?;
    use crate::database::schema::recipes::dsl::*;
    // use crate::database::schema::recipes::dsl::recipes;

    let result = recipes.select(Recipe::as_select()).load(&mut conn).await?;

    Ok(result)
}

pub async fn read_one_recipe(pool: Pool, recipe_id: Uuid) -> Result<Recipe, AppError> {
    let conn = &mut pool.get().await?;
    use crate::database::schema::recipes;

    let result = recipes::table
        .filter(recipes::id.eq(recipe_id))
        .first::<Recipe>(conn)
        .await?;

    dbg!(&result);

    Ok(result)
}

pub async fn update_recipe(
    pool: Pool,
    recipe_id: Uuid,
    new_recipe: PatchRecipe,
) -> Result<Recipe, AppError> {
    let conn = &mut pool.get().await?;
    use crate::database::schema::recipes;

    let result = diesel::update(recipes::table.find(recipe_id))
        .set(&new_recipe)
        .returning(Recipe::as_returning())
        .get_result(conn)
        .await?;
    Ok(result)
}

pub async fn delete_recipe(pool: Pool, recipe_id: Uuid) -> Result<Recipe, AppError> {
    let conn = &mut pool.get().await?;
    use crate::database::schema::recipes;

    let result = diesel::delete(recipes::table.find(recipe_id))
        .returning(Recipe::as_returning())
        .get_result(conn)
        .await?;
    Ok(result)
}

pub async fn create_comment(pool: Pool, comment: Comment) -> Result<Comment, AppError> {
    let mut conn = pool.get().await?;
    use super::schema::comments;

    let result = diesel::insert_into(comments::table)
        .values(comment)
        .returning(Comment::as_returning())
        .get_result(&mut conn)
        .await?;

    Ok(result)
}

pub async fn read_all_comments(pool: Pool) -> Result<Vec<Comment>, AppError> {
    let mut conn = &mut pool.get().await?;
    use crate::database::schema::comments::dsl::*;
    // use crate::database::schema::recipes::dsl::recipes;

    let result = comments
        .select(Comment::as_select())
        .load(&mut conn)
        .await?;

    Ok(result)
}

pub async fn update_comment(
    pool: Pool,
    comment_id: Uuid,
    new_comment: PatchComment,
) -> Result<Comment, AppError> {
    let conn = &mut pool.get().await?;
    use crate::database::schema::comments;

    let result = diesel::update(comments::table.find(comment_id))
        .set(&new_comment)
        .returning(Comment::as_returning())
        .get_result(conn)
        .await?;
    Ok(result)
}

pub async fn delete_comment(pool: Pool, comment_id: Uuid) -> Result<Comment, AppError> {
    let conn = &mut pool.get().await?;
    use crate::database::schema::comments;

    let result = diesel::delete(comments::table.find(comment_id))
        .returning(Comment::as_returning())
        .get_result(conn)
        .await?;
    Ok(result)
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
struct DatabaseConnection(
    bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
);

// I'm guessing this is for await
// also, move to errors?
// this is an extractor, so errors becomes responses. Can't return an anyhow error I think
#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    S: Send + Sync,
    Pool: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);
        let conn = pool.get_owned().await?;

        Ok(Self(conn))
    }
}
