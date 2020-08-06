use rocket;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
mod artist;
mod artist_post;
mod image_proxy;
mod upload;

#[derive(Serialize, Deserialize)]
pub struct SingleResult<T> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<T>,
}

impl<T> SingleResult<T> {
    fn error(error: String) -> Json<SingleResult<T>> {
        Json(SingleResult::<T> {
            success: false,
            error: Some(error),
            result: None,
        })
    }

    fn success(result: T) -> Json<SingleResult<T>> {
        Json(SingleResult::<T> {
            success: true,
            error: None,
            result: Some(result),
        })
    }

    fn make(
        result: Result<T, rocket_contrib::databases::diesel::result::Error>,
    ) -> Json<SingleResult<T>> {
        return match result {
            Ok(json) => SingleResult::success(json),
            Err(e) => SingleResult::error(e.to_string()),
        };
    }
}

pub fn api_routes() -> std::vec::Vec<rocket::Route> {
    routes![artist::route, artist_post::route, image_proxy::route]
}

pub fn proxy_route() -> std::vec::Vec<rocket::Route> {
    routes![image_proxy::route, image_proxy::route_thumb]
}

pub fn upload_routes() -> std::vec::Vec<rocket::Route> {
    routes![upload::route]
}
