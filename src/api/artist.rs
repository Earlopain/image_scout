use crate::api::SingleResult;
use crate::db;
use crate::db::artist::Artist;
use rocket_contrib::json::Json;

#[get("/artist/<name>")]
pub fn route(name: String, conn: db::Connection) -> Json<SingleResult<Artist>> {
    SingleResult::make(db::artist::Artist::get_by_name(&name, &conn))
}
