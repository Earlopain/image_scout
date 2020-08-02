table! {
    artists (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    artist_aliases (id) {
        id -> Integer,
        artist_id -> Integer,
        alias -> Varchar,
    }
}

table! {
    artist_pages (id) {
        id -> Integer,
        artist_id -> Integer,
        url -> Varchar,
        page_type -> Integer,
        added_at -> Timestamp,
        last_update -> Timestamp,
        active -> Bool,
    }
}

table! {
    artist_posts (id) {
        id -> Integer,
        artist_id -> Integer,
        image_id -> Integer,
        source_url -> Varchar,
        direct_url -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

table! {
    images (id) {
        id -> Integer,
        blob -> Longblob,
        width -> Integer,
        height -> Integer,
        perceptual_hash -> Binary,
        file_type -> Varchar,
    }
}

table! {
    page_types (id) {
        id -> Integer,
        name -> Varchar,
        regex -> Varchar,
    }
}

joinable!(artist_aliases -> artists (artist_id));
joinable!(artist_pages -> artists (artist_id));
joinable!(artist_pages -> page_types (page_type));
joinable!(artist_posts -> artists (artist_id));
joinable!(artist_posts -> images (image_id));

allow_tables_to_appear_in_same_query!(
    artists,
    artist_aliases,
    artist_pages,
    artist_posts,
    images,
    page_types,
);
