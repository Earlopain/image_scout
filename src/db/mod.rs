pub mod artist;
pub mod artist_alias;
pub mod artist_page;
pub mod artist_post;

#[database("main")]
pub struct Connection(rocket_contrib::databases::diesel::PgConnection);
