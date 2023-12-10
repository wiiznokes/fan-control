use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=PACKAGE_TYPE");
    if let Ok(packaging_type) = env::var("PACKAGE_TYPE") {
        println!("cargo:rustc-cfg=PACKAGE_TYPE=\"{packaging_type}\"");
    }
}
