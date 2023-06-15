use crate::errors::internal_error;
use crate::Pool;
use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::Json;
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use hyper::StatusCode;
use serde_json::Value;

use crate::Recipe;

// pub fn establish_connection() -> PgConnection {
//     let database_url = env::var("RUST_RECIPE_GALLERY_DATABASE_URL")
//         .expect("RUST_RECIPE_GALLERY_DATABASE_URL must be set");
//     PgConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }

pub async fn post_recipe(
    pool: Pool,
    recipe: Recipe,
) -> Result<Json<Recipe>, (StatusCode, Json<Value>)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
    // use super::schema::recipes::dsl::*;
    use super::schema::recipes;
    // use diesel::prelude::*;
    // let results = recipes.;
    let result = diesel::insert_into(recipes::table)
        .values(recipe)
        .returning(Recipe::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;

    Ok(Json(result))
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
struct DatabaseConnection(
    bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
);

// I'm guessing this is for await
// also, move to errors?
#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    S: Send + Sync,
    Pool: FromRef<S>,
{
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);

        let conn = pool.get_owned().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}
