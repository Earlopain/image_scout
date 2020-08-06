use crate::image_info::ImageInfo;
use crate::schema::upload_cache;
use chrono::{DateTime, Utc};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use upload_cache::columns;

#[derive(Insertable)]
#[table_name = "upload_cache"]
pub struct NewUploadCache<'a> {
    pub blob: &'a Vec<u8>,
    pub perceptual_hash: &'a Vec<u8>,
    pub file_type: &'a str,
    pub added_at: &'a DateTime<Utc>,
}

#[derive(Queryable)]
pub struct UploadCache {
    pub blob: Vec<u8>,
    pub file_type: String,
}

impl UploadCache {
    pub fn create(image: Vec<u8>, connection: &PgConnection) -> Result<i64, diesel::result::Error> {
        match ImageInfo::create_from_vec(image) {
            Ok(info) => {
                let cache = NewUploadCache {
                    blob: info.get_blob(),
                    perceptual_hash: &info.get_perceptual_hash(),
                    file_type: &info.get_format(),
                    added_at: &Utc::now(),
                };

                let inserted = diesel::insert_into(upload_cache::table)
                    .values(&cache)
                    .returning(columns::id)
                    .get_result(connection)?;
                Ok(inserted)
            }
            Err(e) => {
                panic!("Passed invalid image to insert into db\n".to_string() + &e.to_string())
            }
        }
    }

    pub fn get_info(
        search_for: &i64,
        connection: &PgConnection,
    ) -> Result<UploadCache, diesel::result::Error> {
        upload_cache::table
            .select((columns::blob, columns::file_type))
            .filter(columns::id.eq(search_for))
            .first(connection)
    }
}
