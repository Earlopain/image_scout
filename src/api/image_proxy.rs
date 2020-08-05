use crate::db;
use crate::db::artist_post::ArtistPost;
use image;
use rocket::http::ContentType;
use rocket::response::{Body, Response};
use std::convert::TryInto;
use std::io::Cursor;

#[get("/image/<id>")]
pub fn route(id: i64, conn: db::Connection) -> Option<Response<'static>> {
    let image = get_image_from_db(id, &conn);
    match image {
        Ok(post) => Some(craft_response_success(post.blob, post.file_name, post.file_type)),
        Err(_e) => None,
    }
}

#[get("/image/thumb/<id>")]
pub fn route_resize(id: i64, conn: db::Connection) -> Option<Response<'static>> {
    let image = get_image_from_db(id, &conn);
    match image {
        Ok(post) => {
            match resize_image(post.blob, 500, 500) {
                Ok(resized) => Some(craft_response_success(resized, post.file_name, post.file_type)),
                Err(_e) => None,
            }
        },
        Err(_e) => None,
    }
}

fn craft_response_success(blob: Vec<u8>, file_name: String, file_type: String) -> Response<'static> {
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

fn resize_image(img_data: Vec<u8>, width: u32, height: u32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let image = image::load_from_memory(&*img_data).unwrap();
    let mut result: Vec<u8> = Vec::new();
    image.thumbnail(width, height,).write_to(&mut result, image::ImageOutputFormat::Jpeg(85))?;
    Ok(result)
}

fn get_image_from_db(id: i64, conn: &db::Connection) -> Result<ArtistPost, diesel::result::Error> {
    db::artist_post::ArtistPost::get_by_id(&id, &conn)
}
