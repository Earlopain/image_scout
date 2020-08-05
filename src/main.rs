#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

mod api;
mod seeding;

#[database("main")]
pub struct Connection(rocket_contrib::databases::diesel::PgConnection);


#[get("/")]
fn compare() -> Template {
    let mut context = HashMap::<String, String>::new();
    context.insert(String::from("title"), String::from("TEMPLATE TITLE"));
    Template::render("compare", context)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(Connection::fairing())
        .mount("/", routes![compare, seeding::route])
        .mount("/api", api::api_routes())
        .mount("/static", api::proxy_route())
        .mount("/static", StaticFiles::from("static"))
        .launch();
}
