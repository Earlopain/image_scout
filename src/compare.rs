use db::upload_cache::UploadCache;
use rocket::http::ContentType;
use rocket::Data;
use rocket_contrib::templates::tera::Context;
use rocket_contrib::templates::Template;
use rocket_multipart_form_data::mime;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions,
};

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
