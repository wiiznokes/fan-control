# Build

This will depend on the host platform

### Build Linux on Linux
```
sudo apt install make bison flex clang -y
make libsensors
make release
```
### Build Windows on Windows
1. install [dotnet 7](https://dotnet.microsoft.com/en-us/download/dotnet/7.0)
2. run these commands
    ```
    make lhm
    make release
    ```