mod logger;
mod models;

type StdErr = Box<dyn std::error::Error>;

fn main() -> Result<(), StdErr> {
    dotenv::dotenv()?;
    logger::setup_logger()?;

    Ok(())
}
