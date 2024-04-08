use std::env;

fn set_env(var_name: &str) {
    println!("cargo:rerun-if-env-changed={var_name}");

    if let Ok(var) = env::var(var_name) {
        println!("cargo:rustc-cfg={var_name}=\"{var}\"");
    }
}


fn main() {
    set_env("FAN_CONTROL_FORMAT");
    set_env("FAN_CONTROL_VERSION");
    set_env("FAN_CONTROL_COMMIT");
}
