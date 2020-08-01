#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use rocket_contrib::databases::diesel;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[database("main")]
struct DbConn(diesel::MysqlConnection);

#[get("/")]
fn compare() -> Template {
    let mut context = HashMap::<String, String>::new();
    context.insert(String::from("title"), String::from("TEMPLATE TITLE"));
    Template::render("compare", context)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(DbConn::fairing())
        .mount("/", routes![compare])
        .mount("/static", StaticFiles::from("static"))
        .launch();
}
