// @generated automatically by Diesel CLI.

diesel::table! {
    characters (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    ex_gallery (gid) {
        gid -> Int4,
        token -> Text,
        title -> Text,
        title_jpn -> Nullable<Text>,
        category -> Int4,
        rating -> Float8,
        languages -> Array<Nullable<Int4>>,
        female -> Array<Nullable<Int4>>,
        male -> Array<Nullable<Int4>>,
        mixed -> Array<Nullable<Int4>>,
        other -> Array<Nullable<Int4>>,
        rest -> Array<Nullable<Int4>>,
        artists -> Array<Nullable<Int4>>,
        groups -> Array<Nullable<Int4>>,
        cosplayers -> Array<Nullable<Int4>>,
        uploader -> Nullable<Int4>,
        disowned -> Bool,
        parent_gid -> Nullable<Int4>,
        first_gid -> Nullable<Int4>,
        parodies -> Array<Nullable<Int4>>,
        characters -> Array<Nullable<Int4>>,
        thumb -> Text,
        filesize -> Int4,
        filecount -> Int4,
        torrentcount -> Int4,
        torrents -> Text,
        removed -> Nullable<Int4>,
        expunged -> Bool,
        posted -> Int4,
        dumped -> Nullable<Int4>,
    }
}

diesel::table! {
    failed (gid) {
        gid -> Int4,
        reason -> Int4,
    }
}

diesel::table! {
    failed_reasons (id) {
        id -> Int4,
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
    languages (id) {
        id -> Int4,
        language -> Text,
    }
}

diesel::table! {
    p_mixed (gid) {
        gid -> Int4,
        p -> Nullable<Int4>,
    }
}

diesel::table! {
    parodies (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    temp_tags (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    characters,
    ex_gallery,
    failed,
    failed_reasons,
    hitomi_gallery,
    languages,
    p_mixed,
    parodies,
    tags,
    temp_tags,
    users,
);
