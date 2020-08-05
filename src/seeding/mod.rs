use db::artist::Artist;
use crate::Connection;
use rocket::get;

#[get("/seeding")]
pub fn route(conn: Connection) -> Result<(), Box<dyn std::error::Error>> {
    let time = chrono::Utc::now();
    let kenket = Artist::create("kenket", &conn)?;
    kenket.add_alias("tessgarman", &conn)?;
    kenket.add_alias("ketsketch", &conn)?;
    kenket.add_post(
        &1,
        "",
        "https://pbs.twimg.com/media/EV1cvprUwAIDB4F?format=jpg&name=orig",
        &time,
        &conn,
    )?;
    kenket.add_post(
        &2,
        &"",
        &"https://d.facdn.net/art/kenket/1587158288/1587158288.kenket_freefall.jpg",
        &time,
        &conn,
    )?;
    Ok(())
}
