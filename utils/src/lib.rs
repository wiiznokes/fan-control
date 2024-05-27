use std::path::PathBuf;

use cargo_packager_resource_resolver as resource_resolver;
use log::error;

use cached::proc_macro::cached;

// https://github.com/rust-lang/rust/issues/31383

pub const APP_ID: &str = "io.github.wiiznokes.fan-control";

pub const QUALIFIER: &str = "io.github";
pub const ORG: &str = "wiiznokes";
pub const APP: &str = "fan-control";

#[cached]
pub fn resource_dir() -> PathBuf {
    if cfg!(FAN_CONTROL_FORMAT = "flatpak") {
        PathBuf::from(format!("/app/share/{APP_ID}/res"))
    } else {
        match resource_resolver::current_format() {
            Ok(format) => resource_resolver::resources_dir(format).unwrap(),
            Err(e) => {
                if matches!(e, resource_resolver::Error::UnkownPackageFormat) {
                    PathBuf::from("res")
                } else {
                    error!("{e}");
                    panic!()
                }
            }
        }
    }
}
