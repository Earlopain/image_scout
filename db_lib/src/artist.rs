use crate::artist_alias::ArtistAlias;
use crate::artist_page::ArtistPage;
use crate::artist_post::{ArtistPost, ArtistPostNoBlob};
use crate::error::{DbError, InsertImageFromUrlError};
use crate::page_type::PageType;
use crate::schema::artists;
use crate::schema::artists::dsl::*;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[table_name = "artists"]
pub struct NewArtist<'a> {
    pub name: &'a str,
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Queryable)]
pub struct Artist {
    pub id: i64,
    pub name: String,
}

impl Artist {
    pub fn create(artist_name: &str, conn: &PgConnection) -> Result<Artist, DbError> {
        let artist = NewArtist { name: artist_name };
        let inserted = diesel::insert_into(artists::table)
            .values(&artist)
            .get_result(conn)?;
        Ok(inserted)
    }

    pub fn add_alias(
        self: &Self,
        alias: &str,
        conn: &PgConnection,
    ) -> Result<ArtistAlias, DbError> {
        ArtistAlias::create(&self.id, alias, conn)
    }

    pub fn add_page(self: &Self, url: &String, conn: &PgConnection) -> Result<ArtistPage, DbError> {
        match PageType::get_type_from_url(url, conn)? {
            Some(page_type) => ArtistPage::create(&self.id, &page_type, url, conn),
            None => Err(DbError::NotFound),
        }
    }

    pub fn add_post(
        self: &Self,
        page_type: &i64,
        source_url: &str,
        direct_url: &str,
        created_at: &chrono::DateTime<chrono::Utc>,
        conn: &PgConnection,
    ) -> Result<ArtistPostNoBlob, InsertImageFromUrlError> {
        ArtistPost::create(
            &self.id, page_type, source_url, direct_url, created_at, conn,
        )
    }

    pub fn get_by_name(search_for: &str, conn: &PgConnection) -> Result<Artist, DbError> {
        artists::table.filter(name.eq(search_for)).first(conn)
    }
}
