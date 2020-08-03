use crate::db;
use rocket_contrib::json::Json;

#[get("/artist/<name>")]
pub fn route(name: String, conn: db::Connection) -> Json<db::artist::Artist> {
    Json(db::artist::Artist::get_by_name(&name, &conn).unwrap())
}
