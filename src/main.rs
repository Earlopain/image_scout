#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rocket_multipart_form_data;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
mod api;
mod compare;

#[database("main")]
pub struct Connection(rocket_contrib::databases::diesel::PgConnection);

#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", context)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(Connection::fairing())
        .mount("/", routes![index, compare::route])
        .mount("/api", api::api_routes())
        .mount("/static", api::proxy_route())
        .mount("/static", StaticFiles::from("static"))
        .launch();
}
