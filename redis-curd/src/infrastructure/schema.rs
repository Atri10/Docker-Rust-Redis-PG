// @generated automatically by Diesel CLI.

diesel::table! {
    spells (id) {
        id -> Int4,
        name -> Varchar,
        power -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    spells,
);
