// @generated automatically by Diesel CLI.

diesel::table! {
    gallery_pages_meta (gid) {
        gid -> Integer,
        token -> Nullable<Text>,
        parent -> Nullable<Text>,
        visible -> Nullable<Text>,
        favorited -> Nullable<Integer>,
        rated -> Nullable<Integer>,
        uploader_info -> Nullable<Text>,
        gp_tags -> Nullable<Text>,
        newer_versions -> Nullable<Text>,
        image_pages -> Nullable<Text>,
        gp_parsed -> Nullable<Text>,
        uploader_comment -> Nullable<Text>,
        comments_list -> Nullable<Text>,
        crawled -> Nullable<Integer>,
        account -> Nullable<Text>,
        hash -> Nullable<Text>,
    }
}
