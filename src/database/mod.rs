use rocket_contrib::databases::diesel;

#[database("main")]
pub struct Conn(diesel::MysqlConnection);
