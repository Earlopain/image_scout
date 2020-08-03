mod artist;
mod artist_post;

use crate::db::Connection;
use rocket::get;

#[get("/seeding")]
pub fn route(conn: Connection) -> &'static str {
    if artist::insert(&conn).is_err() {
        "Failed to seed artists"
    } else if artist_post::insert(&conn).is_err() {
        "Failed to seed images"
    } else {
        "Success"
    }
}
