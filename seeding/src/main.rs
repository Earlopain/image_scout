use db;
use diesel::Connection;
use diesel::pg::PgConnection;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection();
    let time = chrono::Utc::now();
    let kenket = db::artist::Artist::create("kenket", &conn)?;
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

fn get_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
