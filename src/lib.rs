mod log;

pub use log::*;
pub use log_ori::{log, Level, LevelFilter};

#[cfg(feature = "datetime")]
pub use chrono::Local as __private_chrono_local;
