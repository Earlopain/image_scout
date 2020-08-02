#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use std::collections::HashMap;

mod db;
mod schema;
mod seeding;

#[get("/")]
fn compare() -> Template {
    let mut context = HashMap::<String, String>::new();
    context.insert(String::from("title"), String::from("TEMPLATE TITLE"));
    Template::render("compare", context)
}

#[get("/seeding")]
fn seeding(conn: db::Connection) -> &'static str {
    if seeding::image::insert(conn).is_err() {
        "Failed to seed images"
    } else {
        "Success"
    }
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(db::Connection::fairing())
        .mount("/", routes![compare, seeding])
        .mount("/static", StaticFiles::from("static"))
        .launch();
}
