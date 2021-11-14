#[macro_use]
extern crate diesel;


mod logger;
mod models;
mod schema;


type StdErr = Box<dyn std::error::Error>;

fn main() -> Result<(), StdErr> {
    dotenv::dotenv()?;
    logger::setup_logger()?;

    Ok(())
}
