use crate::db::artist_alias::ArtistAlias;
use crate::schema::artists;
use crate::schema::artists::dsl::*;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "artists"]
pub struct Artist {
    pub id: u32,
    pub name: String,
}

impl Artist {
    pub fn create(artist_name: String, connection: &MysqlConnection) -> Result<(), Box<dyn Error>> {
        let artist = Artist {
            id: 0,
            name: artist_name,
        };

        diesel::insert_into(artists::table)
            .values(&artist)
            .execute(connection)?;
        Ok(())
    }

    pub fn add_alias(
        self: &Self,
        alias: String,
        connection: &MysqlConnection,
    ) -> Result<(), Box<dyn Error>> {
        ArtistAlias::create(self.id, alias, connection)
    }

    pub fn get_by_name(
        search_for: &String,
        connection: &MysqlConnection,
    ) -> Result<Artist, diesel::result::Error> {
        artists::table.filter(name.eq(search_for)).first(connection)
    }
}
