use crate::api::SingleResult;
use db::upload_cache::UploadCache;
use rocket::http::ContentType;
use rocket::Data;
use rocket_contrib::json::Json;
use rocket_multipart_form_data::mime;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions,
};

#[post("/upload", data = "<data>")]
pub fn route(
    content_type: &ContentType,
    data: Data,
    conn: crate::Connection,
) -> Json<SingleResult<UploadCache>> {
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
                return SingleResult::error("The file is too large.".to_string());
            }
            MultipartFormDataError::DataTypeError(_) => {
                return SingleResult::error("The file is not an image".to_string());
            }
            _ => panic!("{:?}", err),
        },
    };

    match multipart_form_data.raw.remove("image") {
        Some(mut image) => SingleResult::make(UploadCache::create(image.remove(0).raw, &conn)),
        None => SingleResult::error("Please input a file".to_string()),
    }
}
