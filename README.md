# fan-control-rs

## References
- UI plans: https://github.com/Ax9D/pw-viz/blob/main/assets/demo.png
- Iced example on canvas: https://github.com/ua-kxie/circe


# Steps
- [ ] finish hardware crate (windows, api)
- [ ] impl UI for managing configs
- [ ] impl UI for removing/adding nodes
- [ ] package Msi, Deb, Rpm, Snap, Flatpak [cargo-bundle](https://github.com/burtonageo/cargo-bundle)
- [ ] CI for packaging
- [ ] work on UI/Iced (develop new widgets when needed, icons, theme, i18n)
- [ ] impl graph behavior
- [ ] settings page

## Repo structure
- [hardware](./hardware/README.md): define an abstraction around the hardware.
- [data](./data/README.md): define structures used in the app (Node, Config), and there logic. Depend on [hardware](./hardware/README.md)
- [ui](./ui/README.md): implement the UI. Depend on [data](./data/README.md)
- the app: integrate all this crates in one executable


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