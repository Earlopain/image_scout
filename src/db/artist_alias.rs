use crate::schema::artist_aliases;
use crate::schema::artist_aliases::dsl;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "artist_aliases"]
pub struct ArtistAlias {
    pub id: u32,
    pub artist_id: u32,
    pub alias: String,
}

impl ArtistAlias {
    pub fn create(
        artist_id: u32,
        alias: String,
        connection: &MysqlConnection,
    ) -> Result<(), Box<dyn Error>> {
        let alias = ArtistAlias {
            id: 0,
            artist_id,
            alias,
        };

        diesel::insert_into(artist_aliases::table)
            .values(&alias)
            .execute(connection)?;
        Ok(())
    }

    pub fn get_by_name(
        search_for: &String,
        connection: &MysqlConnection,
    ) -> Result<std::vec::Vec<ArtistAlias>, diesel::result::Error> {
        artist_aliases::table
            .filter(dsl::alias.eq(search_for))
            .load(connection)
    }
}
