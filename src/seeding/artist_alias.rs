use crate::db::artist::Artist;
use crate::db::Connection;

use std::error::Error;

pub fn insert(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let artist = Artist::get_by_name(&"kenket".to_string(), conn)?;
    artist.add_alias("kenketAlias1".to_string(), conn)?;
    artist.add_alias("kenketAlias2".to_string(), conn)?;
    Ok(())
}
