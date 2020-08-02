use crate::db::artist::Artist;
use crate::db::Connection;

use std::error::Error;

pub fn insert(conn: &Connection) -> Result<(), Box<dyn Error>> {
    Artist::create(
        "kenket".to_string(),
        conn,
    )?;
    Ok(())
}
