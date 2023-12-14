use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=CARGO_PACKAGER_FORMAT");
    if let Ok(packaging_type) = env::var("CARGO_PACKAGER_FORMAT") {
        println!("cargo:rustc-cfg=CARGO_PACKAGER_FORMAT=\"{packaging_type}\"");
    }
}
