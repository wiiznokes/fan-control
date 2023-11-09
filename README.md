# fan-control-rs

## References
- UI plans: https://github.com/Ax9D/pw-viz/blob/main/assets/demo.png
- Iced example on canvas: https://github.com/ua-kxie/circe



## Repo structure
- [hardware](./hardware/README.md): define an abstraction around the hardware.
- [data](./data/README.md): define structures used in the app (Node, Config), and there logic. Depend on [hardware](./hardware/README.md)
- [ui](./ui/README.md): implement the UI. Depend on [data](./data/README.md)
- the app: integrate all this crates into on executable


# Build

## Linux
```
git submodule update --init
make libsensors
```
## Windows
```
dotnet build
```
## Dependencies

#### Ubuntu
```
sudo apt install make bison flex clang -y
```
#### Fedora
```
sudo dnf install make bison flex clang -y