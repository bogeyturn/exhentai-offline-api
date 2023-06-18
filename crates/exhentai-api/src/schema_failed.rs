// @generated automatically by Diesel CLI.

diesel::table! {
    failed (gid) {
        gid -> Integer,
        reason -> Text,
    }
}
