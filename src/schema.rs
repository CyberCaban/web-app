// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::joinable!(files -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    files,
    users,
);
