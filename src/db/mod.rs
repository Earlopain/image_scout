pub mod artist;
pub mod image;

#[database("main")]
pub struct Connection(rocket_contrib::databases::diesel::MysqlConnection);
