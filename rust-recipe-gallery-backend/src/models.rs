use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::database::schema::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    pub id: Uuid,
    pub title: String,
    // // this is dumb. might need another table for ingredients??? Doesn't seem possible to be double not null...
    // pub ingredients: Vec<Option<String>>,
}
