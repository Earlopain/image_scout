use rocket;
mod artist;
mod artist_post;

pub fn routes() -> std::vec::Vec<rocket::Route> {
    return routes![artist::route, artist_post::route];
}
