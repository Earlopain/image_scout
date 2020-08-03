table! {
    artists (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
    }
}

table! {
    artist_aliases (id) {
        id -> Unsigned<Integer>,
        artist_id -> Unsigned<Integer>,
        alias -> Varchar,
    }
}

table! {
    artist_pages (id) {
        id -> Unsigned<Integer>,
        artist_id -> Unsigned<Integer>,
        url -> Varchar,
        page_type -> Unsigned<Integer>,
        added_at -> Timestamp,
        last_update -> Timestamp,
        active -> Bool,
    }
}

table! {
    artist_posts (id) {
        id -> Unsigned<Integer>,
        artist_id -> Unsigned<Integer>,
        page_type -> Unsigned<Integer>,
        source_url -> Varchar,
        direct_url -> Nullable<Varchar>,
        blob -> Longblob,
        width -> Unsigned<Integer>,
        height -> Unsigned<Integer>,
        perceptual_hash -> Binary,
        file_type -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    page_types (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        regex -> Varchar,
    }
}

joinable!(artist_aliases -> artists (artist_id));
joinable!(artist_pages -> artists (artist_id));
joinable!(artist_pages -> page_types (page_type));
joinable!(artist_posts -> artists (artist_id));
joinable!(artist_posts -> page_types (page_type));

allow_tables_to_appear_in_same_query!(
    artists,
    artist_aliases,
    artist_pages,
    artist_posts,
    page_types,
);
