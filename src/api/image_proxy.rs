use crate::db;
use rocket::http::ContentType;
use rocket::response::{Body, Response};
use std::convert::TryInto;
use std::io::Cursor;
use crate::db::artist_post::ArtistPost;

#[get("/image/<id>")]
pub fn route(id: i64, conn: db::Connection) -> Option<Response<'static>> {
    let image = get_image_from_db(id, &conn);
    match image {
        Ok(post) => Some(craft_response_success(post)),
        Err(_e) => None,
    }
}

fn craft_response_success(post: ArtistPost) -> Response<'static> {
    let mut response = Response::new();
    let size = post.blob.len().try_into().unwrap();

    let body = Body::Sized(Cursor::new(post.blob), size);

    response.set_raw_header(
        "Content-Disposition",
        "inline; filename=".to_string() + &post.file_name,
    );
    response.set_header(ContentType::from_extension(&post.file_type).unwrap());
    response.set_raw_body(body);
    response
}

fn get_image_from_db(id: i64, conn: &db::Connection) -> Result<ArtistPost, diesel::result::Error> {
    db::artist_post::ArtistPost::get_by_id(&id, &conn)
}
