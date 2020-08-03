use crate::db;
use rocket_contrib::json::Json;

#[get("/artist_post/<id>")]
pub fn route(id: u32, conn: db::Connection) -> Json<db::artist_post::ArtistPost> {
    Json(db::artist_post::ArtistPost::get_by_id(&id, &conn).unwrap())
}
