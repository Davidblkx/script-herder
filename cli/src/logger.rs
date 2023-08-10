use std::time::SystemTime;

use log::trace;
use script_herder_core::config::{AppConfig, KnownConfigs};

pub fn configure_logger_from_config(config: &AppConfig) -> Result<(), fern::InitError> {
    let level = load_level(config);
    configure_logger(level)
}

pub fn configure_logger(level: log::LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;

    trace!("Logger configured: {:?}", level);

    Ok(())
}

fn load_level(config: &AppConfig) -> log::LevelFilter {
    match config.get::<String>(KnownConfigs::LogLevel) {
        Some(level) => match level.to_lowercase().as_str() {
            "error" => log::LevelFilter::Error,
            "warn" => log::LevelFilter::Warn,
            "info" => log::LevelFilter::Info,
            "debug" => log::LevelFilter::Debug,
            "trace" => log::LevelFilter::Trace,
            "off" => log::LevelFilter::Off,
            _ => log::LevelFilter::Error,
        },
        None => log::LevelFilter::Off,
    }
}