use crate::schema::artist_aliases;
use crate::schema::artist_aliases::dsl;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Insertable)]
#[table_name = "artist_aliases"]
pub struct NewArtistAlias<'a> {
    pub artist_id: &'a i64,
    pub alias: &'a str,
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Queryable)]
pub struct ArtistAlias {
    pub id: i64,
    pub artist_id: i64,
    pub alias: String,
}

impl ArtistAlias {
    pub fn create(
        artist_id: &i64,
        alias: &str,
        connection: &PgConnection,
    ) -> Result<ArtistAlias, Box<dyn Error>> {
        let alias = NewArtistAlias { artist_id, alias };

        let inserted = diesel::insert_into(artist_aliases::table)
            .values(&alias)
            .get_result(connection)?;
        Ok(inserted)
    }

    pub fn get_by_name(
        search_for: &str,
        connection: &PgConnection,
    ) -> Result<std::vec::Vec<ArtistAlias>, diesel::result::Error> {
        artist_aliases::table
            .filter(dsl::alias.eq(search_for))
            .load(connection)
    }
}
