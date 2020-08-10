use crate::error::DbError;
use crate::schema::artist_pages;
use crate::schema::artist_pages::dsl;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[table_name = "artist_pages"]
pub struct NewArtistPage<'a> {
    pub artist_id: &'a i64,
    pub page_type: &'a i64,
    pub url: &'a str,
    pub added_at: DateTime<Utc>,
    pub last_update: DateTime<Utc>,
    pub active: bool,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct ArtistPage {
    pub id: i64,
    pub artist_id: i64,
    pub page_type: i64,
    pub url: String,
    pub added_at: DateTime<Utc>,
    pub last_update: DateTime<Utc>,
    pub active: bool,
}

impl ArtistPage {
    pub fn create(
        artist_id: &i64,
        page_type: &i64,
        url: &str,
        conn: &PgConnection,
    ) -> Result<ArtistPage, DbError> {
        let page = NewArtistPage {
            artist_id,
            page_type,
            url,
            added_at: Utc::now(),
            last_update: DateTime::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            active: true,
        };

        let inserted = diesel::insert_into(artist_pages::table)
            .values(&page)
            .get_result(conn)?;
        Ok(inserted)
    }

    pub fn get_by_artist_id(
        search_for: &i64,
        conn: &PgConnection,
    ) -> Result<std::vec::Vec<ArtistPage>, DbError> {
        artist_pages::table
            .filter(dsl::artist_id.eq(search_for))
            .load(conn)
    }
}
