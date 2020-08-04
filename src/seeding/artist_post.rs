use crate::db::artist_post::ArtistPost;
use crate::db::Connection;
use chrono::Utc;
use std::error::Error;

pub fn insert(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let time = Utc::now();
    ArtistPost::create(
        1,
        1,
        "".to_string(),
        "https://pbs.twimg.com/media/EV1cvprUwAIDB4F?format=jpg&name=orig".to_string(),
        time,
        conn,
    )?;
    ArtistPost::create(
        1,
        2,
        "".to_string(),
        "https://d.facdn.net/art/kenket/1587158288/1587158288.kenket_freefall.jpg".to_string(),
        time,
        conn,
    )?;
    Ok(())
}
