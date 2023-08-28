// @generated automatically by Diesel CLI.

diesel::table! {
    ex_gallery (gid) {
        gid -> Int4,
        title -> Nullable<Text>,
        title_jpn -> Nullable<Text>,
        category -> Nullable<Text>,
        uploader -> Nullable<Text>,
        posted -> Nullable<Int4>,
        thumb -> Nullable<Text>,
        filesize -> Nullable<Int4>,
        filecount -> Nullable<Int4>,
        expunged -> Nullable<Int4>,
        torrentcount -> Nullable<Int4>,
        torrents -> Nullable<Text>,
        token -> Nullable<Text>,
        rating -> Nullable<Float8>,
        artist -> Nullable<Text>,
        group_name -> Nullable<Text>,
        parody -> Nullable<Text>,
        character -> Nullable<Text>,
        female -> Nullable<Text>,
        male -> Nullable<Text>,
        language -> Nullable<Text>,
        mixed -> Nullable<Text>,
        other -> Nullable<Text>,
        cosplayer -> Nullable<Text>,
        rest -> Nullable<Text>,
        parent_gid -> Nullable<Int4>,
        parent_key -> Nullable<Text>,
        first_gid -> Nullable<Int4>,
        first_key -> Nullable<Text>,
        disowned -> Nullable<Int4>,
        removed -> Nullable<Int4>,
        dumped -> Nullable<Int4>,
    }
}

diesel::table! {
    failed (gid) {
        gid -> Int4,
        reason -> Text,
    }
}

diesel::table! {
    hitomi_gallery (id) {
        id -> Int4,
        other_id -> Int4,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        title -> Nullable<Text>,
        jpn_title -> Nullable<Text>,
        tags -> Nullable<Text>,
        artists -> Nullable<Text>,
        groups -> Nullable<Text>,
        parodies -> Nullable<Text>,
        characters -> Nullable<Text>,
        language -> Nullable<Text>,
        language_localname -> Nullable<Text>,
        language_url -> Nullable<Text>,
        languages -> Nullable<Text>,
        related -> Nullable<Text>,
        date -> Nullable<Text>,
        files -> Nullable<Text>,
        file_count -> Nullable<Int4>,
        scene_indexes -> Nullable<Text>,
        video -> Nullable<Text>,
        videofilename -> Nullable<Text>,
        galleryurl -> Nullable<Text>,
        blocked -> Nullable<Text>,
    }
}

diesel::table! {
    p_mixed (gid) {
        gid -> Int4,
        p -> Nullable<Int4>,
    }
}

diesel::table! {
    ratings (id) {
        id -> Int4,
        rating -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(ex_gallery, failed, hitomi_gallery, p_mixed, ratings,);
