use flexi_logger::{detailed_format, Logger};
use log::{error, info, warn};
use anyhow::Result;

pub struct Logg;

impl Logg {
    pub fn start_logger(dir_name: &str) -> Result<()> {
        Logger::try_with_str("info") // Set default log level
            .unwrap()
            .log_to_file(flexi_logger::FileSpec::default().directory(dir_name)) // Log to 'logs' directory
            .format(detailed_format) // Use a detailed format for log lines
            .start()?;
        Ok(())
    }

    pub fn info(msg: String) {
        info!("{}", msg);
    }

    pub fn error(msg: String) {
        error!("{}", msg);
    }

    pub fn warn(msg: String) {
        warn!("{}", msg);
    }
}