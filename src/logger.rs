use log::{info, LevelFilter};

use crate::resources::info::Info;

pub fn init_logging(filter: LevelFilter) {
    env_logger::Builder::new().filter(None, filter).init();
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
