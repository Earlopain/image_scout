use db;
use db::artist_post::ArtistPost;
use db::upload_cache::UploadCache;
use rocket::http::ContentType;
use rocket::response::{Body, Response};
use std::convert::TryInto;
use std::io::Cursor;

#[get("/image/<id>")]
pub fn route(id: i64, conn: crate::Connection) -> Option<Response<'static>> {
    let image = ArtistPost::get_by_id_only_blob(&id, &conn);
    match image {
        Ok(post) => Some(craft_response_success(
            post.blob,
            post.file_name,
            post.file_type,
        )),
        Err(_e) => None,
    }
}

#[get("/image/thumb/<id>")]
pub fn route_thumb(id: i64, conn: crate::Connection) -> Option<Response<'static>> {
    let image = ArtistPost::get_by_id_only_thumb(&id, &conn);
    match image {
        Ok(post) => Some(craft_response_success(
            post.thumb,
            post.file_name,
            post.file_type,
        )),
        Err(_e) => None,
    }
}

//TODO remove from db once request is finished
#[get("/image/uploaded/<id>")]
pub fn route_uploaded(id: i64, conn: crate::Connection) -> Option<Response<'static>> {
    let image = UploadCache::get_info(&id, &conn);
    match image {
        Ok(image) => Some(craft_response_success(
            image.blob,
            id.to_string(),
            image.file_type,
        )),
        Err(_e) => None,
    }
}

fn craft_response_success(
    blob: Vec<u8>,
    file_name: String,
    file_type: String,
) -> Response<'static> {
    let mut response = Response::new();
    let size = blob.len().try_into().unwrap();

    let body = Body::Sized(Cursor::new(blob), size);

    response.set_raw_header(
        "Content-Disposition",
        "inline; filename=".to_string() + &file_name,
    );
    response.set_header(ContentType::from_extension(&file_type).unwrap());
    response.set_raw_body(body);
    response
}
