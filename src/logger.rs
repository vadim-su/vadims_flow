use chrono::Local;
use log::{info, LevelFilter};
use std::io::Write;

use crate::resources::info::Info;

pub fn init_logging(filter: LevelFilter) {
    env_logger::Builder::new()
        .format(|buf, record| {
            let level_color = match record.level() {
                log::Level::Error => "\x1b[31m", // Red
                log::Level::Warn => "\x1b[33m",  // Yellow
                log::Level::Info => "\x1b[32m",  // Green
                log::Level::Debug => "\x1b[34m", // Blue
                log::Level::Trace => "\x1b[37m", // White
            };
            let reset = "\x1b[0m";
            writeln!(
                buf,
                "{} [{}{}{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                level_color,
                record.level(),
                reset,
                record.args()
            )
        })
        .filter(None, filter)
        .init();
}

pub fn log_app_info() {
    let info_data = Info::default();
    info!("══════════════════════════════");
    info!(" Application Info");
    info!("  Version    : {}", info_data.version);
    info!("  Rustc      : {}", info_data.rustc);
    info!("  Build Date : {}", info_data.build_date);
    info!("══════════════════════════════");
}
