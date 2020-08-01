#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use std::collections::HashMap;

mod seeding;
mod database;

#[get("/")]
fn compare() -> Template {
    let mut context = HashMap::<String, String>::new();
    context.insert(String::from("title"), String::from("TEMPLATE TITLE"));
    Template::render("compare", context)
}

#[get("/seeding")]
fn seeding(conn: database::Conn) {
    seeding::image::insert(conn);
}

fn main() {
    rocket::ignite()
    .attach(Template::fairing())
    .attach(database::Conn::fairing())
    .mount("/", routes![compare, seeding])
    .mount("/static", StaticFiles::from("static"))
    .launch();
}
