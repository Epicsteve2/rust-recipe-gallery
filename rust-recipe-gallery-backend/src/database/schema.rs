// @generated automatically by Diesel CLI.

diesel::table! {
    recipes (id) {
        id -> Uuid,
        title -> Varchar,
    }
}
