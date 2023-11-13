# fan-control

# Steps
- [ ] finish hardware crate (windows, upgrade abstraction)
- [ ] Upgrade Value struct (allow different type: °C, %, ...)
- [ ] package Msi, Deb, Rpm, Snap, Flatpak [cargo-bundle](https://github.com/burtonageo/cargo-bundle)
- [ ] CI for packaging
- [ ] change iced to libcosmic (this will enable new widgets, like dropdown)
- [ ] impl UI for managing configs
- [ ] impl UI for removing/adding nodes
- [ ] impl UI graph behavior
- [ ] impl UI settings page
- [ ] icons
- [ ] tray icon support
- [ ] theme
- [ ] i18n

## Repo structure
- [hardware](./hardware/README.md): define an abstraction around the hardware.
- [data](./data/README.md): define structures used in the app (Node, Config), and there logic. Depend on [hardware](./hardware/README.md)
- [ui](./ui/README.md): implement the UI. Depend on [data](./data/README.md) and [hardware](./hardware/README.md)
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