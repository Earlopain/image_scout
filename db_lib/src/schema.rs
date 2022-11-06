// @generated automatically by Diesel CLI.

diesel::table! {
    artist_aliases (id) {
        id -> Int8,
        artist_id -> Int8,
        alias -> Varchar,
    }
}

diesel::table! {
    artist_pages (id) {
        id -> Int8,
        artist_id -> Int8,
        page_type -> Int8,
        url -> Varchar,
        site_user_id -> Varchar,
        site_last_post_id -> Nullable<Varchar>,
        last_update -> Timestamptz,
        added_at -> Timestamptz,
        active -> Bool,
    }
}

diesel::table! {
    artist_posts (id) {
        id -> Int8,
        artist_id -> Int8,
        page_type -> Int8,
        source_url -> Varchar,
        direct_url -> Nullable<Varchar>,
        file_name -> Varchar,
        blob -> Bytea,
        thumb -> Bytea,
        width -> Int8,
        height -> Int8,
        perceptual_hash -> Bytea,
        file_type -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    artists (id) {
        id -> Int8,
        name -> Varchar,
    }
}

diesel::table! {
    page_types (id) {
        id -> Int8,
        name -> Varchar,
        regex -> Varchar,
    }
}

diesel::table! {
    upload_cache (id) {
        id -> Int8,
        blob -> Bytea,
        perceptual_hash -> Bytea,
        file_type -> Varchar,
        added_at -> Timestamptz,
    }
}

diesel::joinable!(artist_aliases -> artists (artist_id));
diesel::joinable!(artist_pages -> artists (artist_id));
diesel::joinable!(artist_pages -> page_types (page_type));
diesel::joinable!(artist_posts -> artists (artist_id));
diesel::joinable!(artist_posts -> page_types (page_type));

diesel::allow_tables_to_appear_in_same_query!(
    artist_aliases,
    artist_pages,
    artist_posts,
    artists,
    page_types,
    upload_cache,
);
