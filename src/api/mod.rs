use rocket;
mod artist;
mod artist_post;
mod image_proxy;

pub fn routes() -> std::vec::Vec<rocket::Route> {
    return routes![artist::route, artist_post::route, image_proxy::route];
}
