use crate::db::artist_alias::ArtistAlias;
use crate::db::artist_page::ArtistPage;
use crate::db::artist_post::{ArtistPost, ArtistPostNoBlob};
use crate::schema::artists;
use crate::schema::artists::dsl::*;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Insertable)]
#[table_name = "artists"]
pub struct NewArtist<'a> {
    pub name: &'a str,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Artist {
    pub id: i64,
    pub name: String,
}

impl Artist {
    pub fn create(
        artist_name: &str,
        connection: &PgConnection,
    ) -> Result<Artist, Box<dyn Error>> {
        let artist = NewArtist { name: artist_name };

        let inserted = diesel::insert_into(artists::table)
            .values(&artist)
            .get_result(connection)?;
        Ok(inserted)
    }

    pub fn add_alias(
        self: &Self,
        alias: &str,
        connection: &PgConnection,
    ) -> Result<ArtistAlias, Box<dyn Error>> {
        ArtistAlias::create(&self.id, alias, connection)
    }

    pub fn add_page(
        self: &Self,
        url: &str,
        connection: &PgConnection,
    ) -> Result<ArtistPage, Box<dyn Error>> {
        //TODO get page_type from url
        let page_type = &1;
        ArtistPage::create(&self.id, page_type, url, connection)
    }

    pub fn add_post(
        self: &Self,
        page_type: &i64,
        source_url: &str,
        direct_url: &str,
        created_at: &chrono::DateTime<chrono::Utc>,
        connection: &PgConnection,
    ) -> Result<ArtistPostNoBlob, Box<dyn Error>> {
        ArtistPost::create(
            &self.id, page_type, source_url, direct_url, created_at, connection,
        )
    }

    pub fn get_by_name(
        search_for: &str,
        connection: &PgConnection,
    ) -> Result<Artist, diesel::result::Error> {
        artists::table.filter(name.eq(search_for)).first(connection)
    }
}
