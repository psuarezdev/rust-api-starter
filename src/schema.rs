// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 150]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
