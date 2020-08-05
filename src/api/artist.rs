use crate::api::SingleResult;
use db;
use db::artist::Artist;
use rocket_contrib::json::Json;

#[get("/artist/<name>")]
pub fn route(name: String, conn: crate::Connection) -> Json<SingleResult<Artist>> {
    SingleResult::make(db::artist::Artist::get_by_name(&name, &conn))
}
