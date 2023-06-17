use crate::errors::AppError;
use crate::Pool;
// use anyhow::Error;
use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};

use crate::Recipe;

pub async fn post_recipe(pool: Pool, recipe: Recipe) -> Result<Recipe, AppError> {
    let mut conn = pool.get().await?;
    use super::schema::recipes;

    let result = diesel::insert_into(recipes::table)
        .values(recipe)
        .returning(Recipe::as_returning())
        .get_result(&mut conn)
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
