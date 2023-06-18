// @generated automatically by Diesel CLI.

diesel::table! {
    ratings (id) {
        id -> Integer,
        same -> Nullable<Text>,
        other_lang -> Nullable<Text>,
        related -> Nullable<Text>,
        rating -> Integer,
    }
}
