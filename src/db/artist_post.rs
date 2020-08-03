use crate::schema::artist_posts;
use chrono::NaiveDateTime;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use image;
use image::GenericImageView;
use img_hash::HasherConfig;
use std::error::Error;

#[derive(Queryable, Insertable)]
#[table_name = "artist_posts"]
pub struct ArtistPost {
    pub id: u32,
    pub artist_id: u32,
    pub page_type: u32,
    pub source_url: String,
    pub direct_url: String,
    pub blob: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub perceptual_hash: Vec<u8>,
    pub file_type: String,
    pub created_at: NaiveDateTime,
}

struct ImageInfo {
    width: u32,
    height: u32,
    perceptual_hash: Vec<u8>,
    file_type: String,
}

impl ArtistPost {
    pub fn create(
        artist_id: u32,
        page_type: u32,
        source_url: String,
        direct_url: String,
        created_at: NaiveDateTime,
        connection: &MysqlConnection,
    ) -> Result<(), Box<dyn Error>> {
        let request = reqwest::blocking::get(&direct_url);
        let image_blob = request?.bytes()?.to_vec();

        let image_info = get_image_info(&image_blob);

        let post = ArtistPost {
            id: 0,
            artist_id,
            page_type,
            source_url,
            direct_url,
            blob: image_blob,
            width: image_info.width,
            height: image_info.height,
            perceptual_hash: image_info.perceptual_hash,
            file_type: image_info.file_type,
            created_at,
        };

        diesel::insert_into(artist_posts::table)
            .values(&post)
            .execute(connection)?;
        Ok(())
    }
}

fn get_image_info(img_data: &Vec<u8>) -> ImageInfo {
    let image = image::load_from_memory(&*img_data).unwrap();
    let dimensions = image.dimensions();
    let image_type = image::guess_format(&*img_data)
        .unwrap()
        .extensions_str()
        .first()
        .unwrap();

    let hasher = HasherConfig::with_bytes_type::<[u8; 32]>().to_hasher();
    let hash = hasher.hash_image(&image);

    ImageInfo {
        width: dimensions.0,
        height: dimensions.1,
        file_type: image_type.to_string(),
        perceptual_hash: hash.as_bytes().to_vec(),
    }
}
