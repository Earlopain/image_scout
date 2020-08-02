use crate::db::image::Image;
use crate::db::Connection;

use std::error::Error;

pub fn insert(conn: &Connection) -> Result<(), Box<dyn Error>> {
    Image::create(
        "https://pbs.twimg.com/media/EV1cvprUwAIDB4F?format=jpg&name=orig".to_string(),
        conn,
    )?;
    Image::create(
        "https://d.facdn.net/art/kenket/1587158288/1587158288.kenket_freefall.jpg".to_string(),
        conn,
    )?;
    Ok(())
}
