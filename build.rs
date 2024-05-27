use std::{env, io};

// https://github.com/mxre/winres/

fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() && std::env::var("PROFILE").unwrap() == "release"
    {
        winres::WindowsResource::new()
            .set_icon("res/windows/app_icon.ico")
            .set_manifest_file("res/windows/manifest.xml")
            .compile()?;
    }
    Ok(())
}
