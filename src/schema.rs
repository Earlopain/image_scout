table! {
    artist (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    artist_alias (id) {
        id -> Integer,
        artist_id -> Integer,
        alias -> Varchar,
    }
}

table! {
    artist_page (id) {
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
    artist_post (id) {
        id -> Integer,
        artist_id -> Integer,
        image_id -> Integer,
        source_url -> Varchar,
        direct_url -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

table! {
    image (id) {
        id -> Integer,
        blob -> Longblob,
        width -> Integer,
        height -> Integer,
        perceptual_hash -> Binary,
        file_type -> Varchar,
    }
}

table! {
    page_type (id) {
        id -> Integer,
        name -> Varchar,
        regex -> Varchar,
    }
}

joinable!(artist_alias -> artist (artist_id));
joinable!(artist_page -> artist (artist_id));
joinable!(artist_page -> page_type (page_type));
joinable!(artist_post -> artist (artist_id));
joinable!(artist_post -> image (image_id));

allow_tables_to_appear_in_same_query!(
    artist,
    artist_alias,
    artist_page,
    artist_post,
    image,
    page_type,
);
