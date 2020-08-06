use crate::image_info::ImageInfo;
use crate::schema::artist_posts;
use artist_posts::columns;
use chrono::{DateTime, Utc};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Insertable)]
#[table_name = "artist_posts"]
pub struct NewArtistPost<'a> {
    pub artist_id: &'a i64,
    pub page_type: &'a i64,
    pub source_url: &'a str,
    pub direct_url: Option<&'a str>,
    pub file_name: &'a str,
    pub blob: &'a Vec<u8>,
    pub thumb: &'a Vec<u8>,
    pub width: &'a i64,
    pub height: &'a i64,
    pub perceptual_hash: &'a Vec<u8>,
    pub file_type: &'a str,
    pub created_at: &'a DateTime<Utc>,
}

//TODO instead of creating almost the same struct twice
//they should somehow inherit their fields
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Queryable)]
pub struct ArtistPostNoBlob {
    pub id: i64,
    pub artist_id: i64,
    pub page_type: i64,
    pub source_url: String,
    pub direct_url: Option<String>,
    pub file_name: String,
    pub width: i64,
    pub height: i64,
    pub perceptual_hash: Vec<u8>,
    pub file_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable)]
pub struct ArtistPostOnlyBlob {
    pub blob: Vec<u8>,
    pub file_name: String,
    pub file_type: String,
}

#[derive(Queryable)]
pub struct ArtistPostOnlyThumb {
    pub thumb: Vec<u8>,
    pub file_name: String,
    pub file_type: String,
}

#[derive(Queryable)]
pub struct ArtistPost {
    pub id: i64,
    pub artist_id: i64,
    pub page_type: i64,
    pub source_url: String,
    pub direct_url: Option<String>,
    pub file_name: String,
    pub blob: Vec<u8>,
    pub thumb: Vec<u8>,
    pub width: i64,
    pub height: i64,
    pub perceptual_hash: Vec<u8>,
    pub file_type: String,
    pub created_at: DateTime<Utc>,
}

impl ArtistPost {
    pub fn create(
        artist_id: &i64,
        page_type: &i64,
        source_url: &str,
        direct_url: &str,
        created_at: &DateTime<Utc>,
        connection: &PgConnection,
    ) -> Result<ArtistPostNoBlob, Box<dyn Error>> {
        let info = ImageInfo::create_from_url(direct_url)?;
        let post = NewArtistPost {
            artist_id,
            page_type,
            source_url,
            file_name: info.get_file_name(),
            direct_url: Some(direct_url),
            blob: info.get_blob(),
            thumb: &info.get_thumbnail(500, 500)?,
            width: &i64::from(info.get_width()),
            height: &i64::from(info.get_height()),
            perceptual_hash: &info.get_perceptual_hash(),
            file_type: info.get_format(),
            created_at,
        };

        let inserted = diesel::insert_into(artist_posts::table)
            .values(&post)
            .returning((
                columns::id,
                columns::artist_id,
                columns::page_type,
                columns::source_url,
                columns::direct_url,
                columns::file_name,
                columns::width,
                columns::height,
                columns::perceptual_hash,
                columns::file_type,
                columns::created_at,
            ))
            .get_result(connection)?;
        Ok(inserted)
    }

    pub fn get_by_id(
        search_for: &i64,
        connection: &PgConnection,
    ) -> Result<ArtistPost, diesel::result::Error> {
        artist_posts::table
            .filter(columns::id.eq(search_for))
            .first(connection)
    }

    pub fn get_by_id_no_blob(
        search_for: &i64,
        connection: &PgConnection,
    ) -> Result<ArtistPostNoBlob, diesel::result::Error> {
        artist_posts::table
            .select((
                columns::id,
                columns::artist_id,
                columns::page_type,
                columns::source_url,
                columns::direct_url,
                columns::file_name,
                columns::width,
                columns::height,
                columns::perceptual_hash,
                columns::file_type,
                columns::created_at,
            ))
            .filter(columns::id.eq(search_for))
            .first(connection)
    }

    pub fn get_by_id_only_blob(
        search_for: &i64,
        connection: &PgConnection,
    ) -> Result<ArtistPostOnlyBlob, diesel::result::Error> {
        artist_posts::table
            .select((columns::blob, columns::file_name, columns::file_type))
            .filter(columns::id.eq(search_for))
            .first(connection)
    }

    pub fn get_by_id_only_thumb(
        search_for: &i64,
        connection: &PgConnection,
    ) -> Result<ArtistPostOnlyThumb, diesel::result::Error> {
        artist_posts::table
            .select((columns::thumb, columns::file_name, columns::file_type))
            .filter(columns::id.eq(search_for))
            .first(connection)
    }
}
