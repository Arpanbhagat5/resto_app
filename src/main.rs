// For rocket macros to work, we need the following, and this requires nightly rust
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

mod logger;
mod models;
mod schema;
mod db;
mod routes;

type StdErr = Box<dyn std::error::Error>;

// Basic index route
#[rocket::get("/")]
fn index() -> &'static str {
   "Hello, Resto App!"
}

fn main() -> Result<(), StdErr> {
    // couldn't handle this so need to set env var separately before running app locally
    // dotenv::dotenv()?;
    logger::setup_logger()?;

    let db = db::Db::connect()?;

    rocket::ignite()
        .manage(db)
        .mount("/", rocket::routes![index])
        .mount("/api", routes::api())
        .launch();

    Ok(())
}
