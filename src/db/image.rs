use crate::schema::images;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use std::error::Error;

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

impl Image {
    pub fn create(url: String, connection: &MysqlConnection) -> Result<(), Box<dyn Error>> {
        let request = reqwest::blocking::get(&url);
        let image = request?.bytes()?.to_vec();

        let image = Image {
            id: 0,
            blob: image,
            width: 0,
            height: 0,
            perceptual_hash: Vec::new(),
            file_type: "jpg".to_string(),
        };

        diesel::insert_into(images::table)
            .values(&image)
            .execute(connection)?;
        Ok(())
    }
}
