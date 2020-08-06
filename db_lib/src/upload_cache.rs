use crate::schema::upload_cache;
use chrono::{DateTime, Utc};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use upload_cache::columns;

#[derive(Insertable)]
#[table_name = "upload_cache"]
pub struct NewUploadCache<'a> {
    pub blob: &'a Vec<u8>,
    pub added_at: &'a DateTime<Utc>,
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Queryable)]
pub struct UploadCache {
    pub id: i64,
    pub added_at: DateTime<Utc>,
}

impl UploadCache {
    pub fn create(
        image: &Vec<u8>,
        connection: &PgConnection,
    ) -> Result<UploadCache, diesel::result::Error> {
        let cache = NewUploadCache {
            blob: image,
            added_at: &Utc::now(),
        };

        let inserted = diesel::insert_into(upload_cache::table)
            .values(&cache)
            .returning((columns::id, columns::added_at))
            .get_result(connection)?;
        Ok(inserted)
    }

    pub fn get_blob(
        search_for: &i64,
        connection: &PgConnection,
    ) -> Result<Vec<u8>, diesel::result::Error> {
        upload_cache::table
            .select(columns::blob)
            .filter(columns::id.eq(search_for))
            .first(connection)
    }
}
