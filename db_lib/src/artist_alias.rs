use crate::error::DbError;
use crate::schema::artist_aliases;
use crate::schema::artist_aliases::dsl;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[table_name = "artist_aliases"]
pub struct NewArtistAlias<'a> {
    pub artist_id: &'a i64,
    pub alias: &'a str,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct ArtistAlias {
    pub id: i64,
    pub artist_id: i64,
    pub alias: String,
}

impl ArtistAlias {
    pub fn create(
        artist_id: &i64,
        alias: &str,
        conn: &PgConnection,
    ) -> Result<ArtistAlias, DbError> {
        let alias = NewArtistAlias { artist_id, alias };

        let inserted = diesel::insert_into(artist_aliases::table)
            .values(&alias)
            .get_result(conn)?;
        Ok(inserted)
    }

    pub fn get_by_name(
        search_for: &str,
        conn: &PgConnection,
    ) -> Result<std::vec::Vec<ArtistAlias>, DbError> {
        artist_aliases::table
            .filter(dsl::alias.eq(search_for))
            .load(conn)
    }
}
