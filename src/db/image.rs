use crate::schema::images;
use diesel;
use diesel::mysql::MysqlConnection;
use image;
use img_hash;
use diesel::prelude::*;
use std::error::Error;
use image::GenericImageView;
use img_hash::HasherConfig;

#[derive(Queryable, Insertable)]
#[table_name = "images"]
pub struct Image {
    pub id: u32,
    pub blob: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub perceptual_hash: Vec<u8>,
    pub file_type: String,
}

struct ImageInfo {
    width: u32,
    height: u32,
    perceptual_hash: Vec<u8>,
    file_type: String,
}

impl Image {
    pub fn create(url: String, connection: &MysqlConnection) -> Result<(), Box<dyn Error>> {
        let request = reqwest::blocking::get(&url);
        let image = request?.bytes()?.to_vec();

        let image_info = get_image_info(&image);

        let image = Image {
            id: 0,
            blob: image,
            width: image_info.width,
            height: image_info.height,
            perceptual_hash: image_info.perceptual_hash,
            file_type: image_info.file_type,
        };

        diesel::insert_into(images::table)
            .values(&image)
            .execute(connection)?;
        Ok(())
    }
}


fn get_image_info(img_data: &Vec<u8>) -> ImageInfo {
    let image = image::load_from_memory(&*img_data).unwrap();
    let dimensions = image.dimensions();
    let image_type = image::guess_format(&*img_data).unwrap().extensions_str().first().unwrap();

    let hasher = HasherConfig::with_bytes_type::<[u8; 32]>().to_hasher();
    let hash = hasher.hash_image(&image);

    ImageInfo {
        width: dimensions.0,
        height: dimensions.1,
        file_type: image_type.to_string(),
        perceptual_hash: hash.as_bytes().to_vec(),
    }
}
