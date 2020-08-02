use crate::schema::artists;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use std::error::Error;

#[derive(Queryable, Insertable)]
#[table_name = "artists"]
pub struct Artist {
    pub id: i32,
    pub name: String,
}

impl Artist {
    pub fn create(name: String, connection: &MysqlConnection) -> Result<(), Box<dyn Error>> {
        let artist = Artist {
            id: 0,
            name: name,
        };

        diesel::insert_into(artists::table)
            .values(&artist)
            .execute(connection)?;
        Ok(())
    }
}
