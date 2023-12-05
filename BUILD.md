# Build

### Linux
```
sudo apt install make bison flex clang -y
make libsensors
cargo run --release
```
### Windows
1. install [dotnet 7](https://dotnet.microsoft.com/en-us/download/dotnet/7.0)
2. run these commands
    ```
    make lhm
    cargo run --release
    ```