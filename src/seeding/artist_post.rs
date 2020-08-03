use crate::db::artist_post::ArtistPost;
use crate::db::Connection;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::error::Error;

pub fn insert(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let d = NaiveDate::from_ymd(2015, 6, 3);
    let t = NaiveTime::from_hms_milli(12, 34, 56, 789);
    let dt = NaiveDateTime::new(d, t);
    ArtistPost::create(
        1,
        1,
        "".to_string(),
        "https://pbs.twimg.com/media/EV1cvprUwAIDB4F?format=jpg&name=orig".to_string(),
        dt,
        conn,
    )?;
    ArtistPost::create(
        1,
        1,
        "".to_string(),
        "https://d.facdn.net/art/kenket/1587158288/1587158288.kenket_freefall.jpg".to_string(),
        dt,
        conn,
    )?;
    Ok(())
}
