use std::process::Command;


fn main() {

    if cfg!(target_os = "linux") {

        let output = Command::new("git")
            .arg("submodule")
            .arg("update")
            .arg("--init")
            .output()
            .expect("Failed to fetch submodules");

        if !output.status.success() {
            eprintln!("Error executing 'git submodule update --init': {:?}", output);
            println!("Maybe git is not installed ?");
            std::process::exit(1);
        }

        let output = Command::new("make")
            .arg("-C")
            .arg("./libsensors")
            .arg("install")
            .arg("PREFIX=./../libsensors_build")
            .arg("ETCDIR=./../etc")
            .output()
            .expect("Failed to execute 'make' command");

        if !output.status.success() {
            eprintln!("Error trying to build libsensors: {:?}", output);
            println!("Maybe make, bison, flex, or clang is not installed ?");
            std::process::exit(1);
        }

    }
}