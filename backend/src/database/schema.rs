// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Uuid,
        recipe_id -> Uuid,
        comment -> Text,
    }
}

diesel::table! {
    recipes (id) {
        id -> Uuid,
        title -> Varchar,
        ingredients -> Text,
        body -> Text,
    }
}

diesel::joinable!(comments -> recipes (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    recipes,
);
