use std::fs::OpenOptions;
use chrono::Local;
use fern::Dispatch;
use log::LevelFilter;

// Function to set up logging
pub fn setup_logging() -> Result<(), fern::InitError> {
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("application_errors.log")?; // Specify your log file name

    Dispatch::new()
        // Set the default minimum log level for everything
        .level(LevelFilter::Info)
        // Filter out excessive logs from specific modules if needed
        // .level_for("rocket", LevelFilter::Warn)
        // Format the log messages
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        // Do not send to standard output (optional, you can also chain both)
        .chain(std::io::stdout())
        // Chain the log file appender
        .chain(log_file)
        .apply()?; // Apply the configuration

    Ok(())
}