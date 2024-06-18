use std::{
    env,
    fs::File,
    io::{self, Read},
};

// https://github.com/mxre/winres/

fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() && std::env::var("PROFILE").unwrap() == "release"
    {
        winres::WindowsResource::new()
            .set_icon("res/windows/app_icon.ico")
            .set_manifest_file("res/windows/manifest.xml")
            .compile()?;
    }

    println!("cargo:rerun-if-changed=VERSION");

    let mut file = File::open("VERSION")?;

    let mut version = String::new();

    file.read_to_string(&mut version)?;

    println!("cargo:rustc-env=FAN_CONTROL_VERSION={}", version);

    Ok(())
}
