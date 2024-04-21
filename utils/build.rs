use std::env;

fn main() {

    println!("cargo:rerun-if-env-changed=FAN_CONTROL_FORMAT");

    if let Ok(var) = env::var("FAN_CONTROL_FORMAT") {
        println!("cargo:rustc-cfg=FAN_CONTROL_FORMAT=\"{}\"", var);
    }
   
}