// For rocket macros we need the following, and this requires nightly rust
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
    dotenv::dotenv()?;
    logger::setup_logger()?;

    let db = db::Db::connect()?;

    rocket::ignite()
        .manage(db)
        .mount("/", rocket::routes![index])
        .mount("/api", routes::api())
        .launch();

    Ok(())
}
