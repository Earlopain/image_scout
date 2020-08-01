#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use clap::{App, Arg};
use rocket_contrib::databases::diesel;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use std::collections::HashMap;

mod seeding;

#[database("main")]
struct DbConn(diesel::MysqlConnection);

#[get("/")]
fn compare() -> Template {
    let mut context = HashMap::<String, String>::new();
    context.insert(String::from("title"), String::from("TEMPLATE TITLE"));
    Template::render("compare", context)
}

fn main() {
    let matches = App::new(std::option_env!("CARGO_PKG_NAME").unwrap_or(""))
        .version(std::option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .author(std::option_env!("CARGO_PKG_AUTHORS").unwrap_or(""))
        .about(std::option_env!("CARGO_PKG_DESCRIPTION").unwrap_or(""))
        .arg(
            Arg::with_name("seed")
                .long("seed")
                .help("Insert testing data into the database"),
        )
        .get_matches();

    if matches.is_present("seed") {
        seeding::start();
    } else {
        rocket::ignite()
            .attach(Template::fairing())
            .attach(DbConn::fairing())
            .mount("/", routes![compare])
            .mount("/static", StaticFiles::from("static"))
            .launch();
    }
}
