use crate::schema::images;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[table_name = "images"]
pub struct Image {
    pub id: i32,
    pub blob: Vec<u8>,
    pub width: i32,
    pub height: i32,
    pub perceptual_hash: Vec<u8>,
    pub file_type: String,
}

impl Image {
    pub fn create(url: String, connection: &MysqlConnection) -> bool {
        let request = reqwest::blocking::get(&url);
        let image = request.unwrap().bytes().unwrap().to_vec();

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
            .execute(connection)
            .expect("Error creating new hero");

        return true;
    }
}
