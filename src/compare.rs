use db::upload_cache::UploadCache;
use rocket::http::ContentType;
use rocket::Data;
use rocket_contrib::templates::Template;
use rocket_multipart_form_data::mime;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions,
};
use std::collections::HashMap;

#[post("/compare", data = "<data>")]
pub fn route(content_type: &ContentType, data: Data, conn: crate::Connection) -> Template {
    insert_image_into_db(content_type, data, conn);
    let mut context = HashMap::<String, String>::new();
    context.insert(String::from("title"), String::from("TEMPLATE TITLE"));
    Template::render("compare", context)
}

fn insert_image_into_db(
    content_type: &ContentType,
    data: Data,
    conn: crate::Connection,
) -> Result<UploadCache, String> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::raw("image")
            .size_limit(32 * 1024 * 1024)
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
    ]);

    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => match err {
            MultipartFormDataError::DataTooLargeError(_) => {
                return Err("The file is too large.".to_string());
            }
            MultipartFormDataError::DataTypeError(_) => {
                return Err("The file is not an image".to_string());
            }
            _ => panic!("{:?}", err),
        },
    };

    match multipart_form_data.raw.remove("image") {
        Some(mut image) => match UploadCache::create(image.remove(0).raw, &conn) {
            Ok(cache) => Ok(cache),
            Err(e) => Err(e.to_string()),
        },
        None => Err("Please input a file".to_string()),
    }
}
