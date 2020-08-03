use crate::db;
use chrono::NaiveDateTime;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct ApiArtistPost {
    pub id: u32,
    pub artist_id: u32,
    pub page_type: u32,
    pub source_url: String,
    pub direct_url: Option<String>,
    pub width: u32,
    pub height: u32,
    pub perceptual_hash: String,
    pub file_type: String,
    pub created_at: NaiveDateTime,
}

#[get("/artist_post/<id>")]
pub fn route(id: u32, conn: db::Connection) -> Json<ApiArtistPost> {
    let post = db::artist_post::ArtistPost::get_by_id_no_blob(&id, &conn).unwrap();
    Json(ApiArtistPost {
        id: post.id,
        artist_id: post.artist_id,
        page_type: post.page_type,
        source_url: post.source_url,
        direct_url: post.direct_url,
        width: post.width,
        height: post.height,
        perceptual_hash: base64::encode(&post.perceptual_hash),
        file_type: post.file_type,
        created_at: post.created_at,
    })
}
