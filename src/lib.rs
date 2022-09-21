use once_cell::sync::OnceCell;

#[macro_export]
macro_rules! trace {
    ($x:expr) => {
        log::trace!(target:module_path!(), "{:?}",  $x)
    };
    ($($x:expr),*) => {
        log::trace!(target:module_path!(), $($x),*)
    };
}

#[macro_export]
macro_rules! debug {
    ($x:expr) => {
        log::debug!(target:module_path!(), "{:?}",  $x)
    };
    ($($x:expr),*) => {
        log::debug!(target:module_path!(), $($x),*)
    };
}

#[macro_export]
macro_rules! info {
    ($x:expr) => {
        log::info!(target:module_path!(), "{:?}",  $x)
    };
    ($($x:expr),*) => {
        log::info!(target:module_path!(), $($x),*)
    };
}

#[macro_export]
macro_rules! warn {
    ($x:expr) => {
        log::warn!(target:module_path!(), "{:?}",  $x)
    };
    ($($x:expr),*) => {
        log::warn!(target:module_path!(), $($x),*)
    };
}

#[macro_export]
macro_rules! error {
    ($x:expr) => {
        log::error!(target:module_path!(), "{:?}",  $x)
    };
    ($($x:expr),*) => {
        log::error!(target:module_path!(), $($x),*)
    };
}

pub static LOG_CONFIG: OnceCell<LogConfig> = OnceCell::new();

pub struct LogConfig;

pub fn prepare_log() {

    prepare_log_with_file("log4rs_config.yaml");
}


pub fn prepare_log_with_file(log_file: &str) {
    LOG_CONFIG.get_or_init(|| {
        log4rs::init_file(log_file, Default::default()).unwrap();
        LogConfig
    });
}
