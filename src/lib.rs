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

pub fn prepare_log() {
    log4rs::init_file("log4rs_config.yaml", Default::default()).unwrap();
}