use crate::db::artist::Artist;
use crate::db::Connection;
use rocket::get;

#[get("/seeding")]
pub fn route(conn: Connection) -> Result<(), Box<dyn std::error::Error>> {
    let time = chrono::Utc::now();
    let kenket = Artist::create("kenket".to_string(), &conn)?;
    kenket.add_alias("kenketAlias1".to_string(), &conn)?;
    kenket.add_alias("kenketAlias2".to_string(), &conn)?;
    kenket.add_post(
        1,
        "".to_string(),
        "https://pbs.twimg.com/media/EV1cvprUwAIDB4F?format=jpg&name=orig".to_string(),
        time,
        &conn,
    )?;
    kenket.add_post(
        2,
        "".to_string(),
        "https://d.facdn.net/art/kenket/1587158288/1587158288.kenket_freefall.jpg".to_string(),
        time,
        &conn,
    )?;
    Ok(())
}
