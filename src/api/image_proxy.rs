use crate::db;
use rocket::http::ContentType;
use rocket::response::{Body, Response};
use std::convert::TryInto;
use std::io::Cursor;

#[get("/image/<id>")]
pub fn route(id: i64, conn: db::Connection) -> Response<'static> {
    let image = db::artist_post::ArtistPost::get_by_id(&id, &conn).unwrap();
    let mut response = Response::new();

    let size = image.blob.len().try_into().unwrap();

    let body = Body::Sized(Cursor::new(image.blob), size);

    response.set_raw_header(
        "Content-Disposition",
        "inline; filename=".to_string() + &image.file_name,
    );
    response.set_header(ContentType::from_extension(&image.file_type).unwrap());
    response.set_raw_body(body);
    response
}
