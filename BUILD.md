# Build

You will need to install [rust](https://www.rust-lang.org/tools/install).

Also, the project use [just](https://github.com/casey/just) as a command runner. It is similar to make, but more simple. There are multiple ways to install it. You can build it with `cargo install just`.

### Linux

```shell
git clone https://github.com/wiiznokes/fan-control.git
cd fan-control
sudo apt install make bison flex clang -y
just libsensors
cargo run --release # or just deb for building a deb package
```

### Windows

1. install [dotnet 8](https://dotnet.microsoft.com/en-us/download/dotnet/8.0)
2. run these commands
   ```shell
   git clone https://github.com/wiiznokes/fan-control.git
   cd fan-control
   just lhm
   cargo run --release # or just nsis
   ```
