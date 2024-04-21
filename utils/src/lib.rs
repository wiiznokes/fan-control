use std::{path::PathBuf, process::Command};

use cargo_packager_resource_resolver as resource_resolver;
use log::error;

use cached::proc_macro::cached;

#[cached]
pub fn resource_dir() -> PathBuf {

    let _ = Command::new("ls").spawn();
    let _ = Command::new("pwd").spawn();

    if cfg!(FAN_CONTROL_FORMAT = "flatpak") {
        println!("flatpak");
        PathBuf::from("/resource")
    } else {
        println!("not flatpak");
        match resource_resolver::current_format() {
            Ok(format) => resource_resolver::resources_dir(format).unwrap(),
            Err(e) => {
                if matches!(e, resource_resolver::Error::UnkownPackageFormat) {
                    PathBuf::from("resource")
                } else {
                    error!("{e}");
                    panic!()
                }
            }
        }
    }
}
