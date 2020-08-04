use crate::schema::artist_aliases;
use crate::schema::artist_aliases::dsl;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Insertable)]
#[table_name = "artist_aliases"]
pub struct NewArtistAlias {
    pub artist_id: i64,
    pub alias: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct ArtistAlias {
    pub id: i64,
    pub artist_id: i64,
    pub alias: String,
}

impl ArtistAlias {
    pub fn create(
        artist_id: i64,
        alias: String,
        connection: &PgConnection,
    ) -> Result<(), Box<dyn Error>> {
        let alias = NewArtistAlias { artist_id, alias };

        diesel::insert_into(artist_aliases::table)
            .values(&alias)
            .execute(connection)?;
        Ok(())
    }

    pub fn get_by_name(
        search_for: &String,
        connection: &PgConnection,
    ) -> Result<std::vec::Vec<ArtistAlias>, diesel::result::Error> {
        artist_aliases::table
            .filter(dsl::alias.eq(search_for))
            .load(connection)
    }
}
