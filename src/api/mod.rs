use rocket;
mod artist;

pub fn routes() -> std::vec::Vec<rocket::Route> {
    return routes![artist::route];
}
