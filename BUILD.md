# Build

You will need to install [rust](https://www.rust-lang.org/tools/install).

Also, the project use [just](https://github.com/casey/just) as a command runner. It is similar to make, but more simple. There are multiple ways to install it. You can build it with `cargo install just`.

### Linux
```shell
sudo apt install make bison flex clang -y
just libsensors
cargo run --release # or just deb for building a deb package
```
### Windows
1. install [dotnet 7](https://dotnet.microsoft.com/en-us/download/dotnet/7.0)
2. run these commands
    ```shell
    just lhm
    cargo run --release # or just nsis 
    ```