mod artist;
mod image;

use crate::db::Connection;
use rocket::get;

#[get("/seeding")]
pub fn route(conn: Connection) -> &'static str {
    if artist::insert(&conn).is_err() {
        "Failed to seed artists"
    } else if image::insert(&conn).is_err() {
        "Failed to seed images"
    } else {
        "Success"
    }
}
