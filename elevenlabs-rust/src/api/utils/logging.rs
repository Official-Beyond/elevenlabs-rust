use log::{info, warn, error};

pub fn setup_logging(level: log::LevelFilter) {
    env_logger::builder().filter(None, level).init();
}

pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_warning(message: &str) {
    warn!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}
