use crate::api::SingleResult;
use db;
use chrono::{DateTime, Utc};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

//TODO copying struct fields must be easier than this
#[derive(Serialize, Deserialize)]
pub struct ApiArtistPost {
    pub id: i64,
    pub artist_id: i64,
    pub page_type: i64,
    pub source_url: String,
    pub direct_url: Option<String>,
    pub file_name: String,
    pub width: i64,
    pub height: i64,
    pub perceptual_hash: String,
    pub file_type: String,
    pub created_at: DateTime<Utc>,
}

#[get("/artist_post/<id>")]
pub fn route(id: i64, conn: crate::Connection) -> Json<SingleResult<ApiArtistPost>> {
    let result = db::artist_post::ArtistPost::get_by_id_no_blob(&id, &conn);

    return match result {
        Ok(post) => SingleResult::success(ApiArtistPost {
            id: post.id,
            artist_id: post.artist_id,
            page_type: post.page_type,
            source_url: post.source_url,
            direct_url: post.direct_url,
            file_name: post.file_name,
            width: post.width,
            height: post.height,
            perceptual_hash: base64::encode(&post.perceptual_hash),
            file_type: post.file_type,
            created_at: post.created_at,
        }),
        Err(e) => SingleResult::error(e.to_string()),
    };
}
