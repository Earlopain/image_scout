use crate::db;
use rocket::http::ContentType;
use rocket::response::content::Content;

#[get("/image/<id>")]
pub fn route(id: u32, conn: db::Connection) -> Content<Vec<u8>> {
    let image = db::artist_post::ArtistPost::get_by_id(&id, &conn).unwrap();
    Content(
        ContentType::from_extension(&image.file_type).unwrap(),
        image.blob,
    )
}
