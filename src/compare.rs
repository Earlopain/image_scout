use db::error::InsertImageFromUrlError;
use db::upload_cache::UploadCache;
use rocket::http::ContentType;
use rocket::Data;
use rocket_contrib::templates::tera::Context;
use rocket_contrib::templates::Template;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions,
};

enum MultipartContent {
    Blob(Vec<u8>),
    Url(String),
}

#[post("/compare", data = "<data>")]
pub fn route(content_type: &ContentType, data: Data, conn: crate::Connection) -> Template {
    //TODO error template
    let upload = insert_image_into_db(content_type, data, &conn).unwrap();

    let similar = upload.get_similar_artist_posts(&conn).unwrap();

    let mut context = Context::new();
    context.insert("title", "TEMPLATE TITLE");
    context.insert("uploaded_image_id", &upload.id.to_string());
    context.insert("similar_images", &similar);
    Template::render("compare", context)
}

fn insert_image_into_db(
    content_type: &ContentType,
    data: Data,
    conn: &crate::Connection,
) -> Result<UploadCache, InsertImageFromUrlError> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::raw("image").size_limit(32 * 1024 * 1024),
        MultipartFormDataField::text("url"),
    ]);

    let form_data =
        get_image_type_from_multiform(MultipartFormData::parse(content_type, data, options));

    let upload_cache = match form_data {
        Some(data_type) => match data_type {
            MultipartContent::Blob(data) => UploadCache::create_from_vec(data, &conn)?,
            MultipartContent::Url(url) => UploadCache::create_from_url(&url, &conn)?,
        },
        None => panic!("You must provide either a image or a url"),
    };
    Ok(upload_cache)
}

fn get_image_type_from_multiform(
    data: Result<MultipartFormData, MultipartFormDataError>,
) -> Option<MultipartContent> {
    let mut multipart_form_data = match data {
        Ok(multipart_form_data) => multipart_form_data,
        Err(_err) => return None,
    };

    if let Some(mut url) = multipart_form_data.texts.remove("url") {
        let text = url.remove(0).text;
        if text.len() > 0 {
            return Some(MultipartContent::Url(text));
        }
    }
    if let Some(mut image) = multipart_form_data.raw.remove("image") {
        let blob = image.remove(0).raw;
        if blob.len() > 0 {
            return Some(MultipartContent::Blob(blob));
        }
    }
    None
}
