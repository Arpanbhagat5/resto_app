use std::fs;
use std::env;

use log::{debug, error, info, trace, warn};

/// This implementation is almost 100% copied from docs examples and some other references
pub fn setup_logger() -> Result<(), fern::InitError> {
    // get log level from .env
    let log_level = env::var("LOG_LEVEL").unwrap_or("INFO".into());
    let log_level = log_level
        .parse::<log::LevelFilter>()
        .unwrap_or(log::LevelFilter::Info);

    // Boiler plate from 'fern' create docs
    let mut builder = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        // log to stderr
        .chain(std::io::stderr());

    // also log to file, is provided via .env
    if let Ok(log_file) = env::var("LOG_FILE") {
        let log_file = fs::File::create(log_file)?;
        builder = builder.chain(log_file);
    }

    // globally apply logger
    builder.apply()?;

    // Init log level text
    trace!("TRACE content");
    debug!("DEBUG content");
    info!("INFO content");
    warn!("WARN content");
    error!("ERROR content");

    Ok(())
}