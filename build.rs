use std::{
    env,
    io::{self},
};

fn set_env(var_name: &str) {
    println!("cargo:rerun-if-env-changed={var_name}");

    if let Ok(var) = env::var(var_name) {
        println!("cargo:rustc-cfg={var_name}=\"{var}\"");
    }
}

fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() && std::env::var("PROFILE").unwrap() == "release"
    {
        // https://github.com/mxre/winres/
        winres::WindowsResource::new()
            .set_icon("res/windows/app_icon.ico")
            .set_manifest_file("res/windows/manifest.xml")
            .compile()?;
    }

    set_env("FAN_CONTROL_VERSION");
    set_env("FAN_CONTROL_COMMIT");

    Ok(())
}
