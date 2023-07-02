// @generated automatically by Diesel CLI.

diesel::table! {
    movies (id) {
        id -> Integer,
        #[max_length = 100]
        title -> Varchar,
        release_date -> Nullable<Date>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 50]
        genre -> Nullable<Varchar>,
        icon -> Nullable<Text>,
    }
}
