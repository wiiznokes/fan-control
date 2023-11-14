# fan-control

# Steps
- [x] finish hardware crate (windows, upgrade abstraction)
- [ ] package Msi, Deb, Rpm, Snap, Flatpak (will require works on [cargo-bundle](https://github.com/burtonageo/cargo-bundle))
- [ ] CI for packaging
- [ ] change iced to libcosmic (this will enable new widgets, like dropdown)
- [ ] impl UI for managing configs
- [ ] impl UI for removing/adding nodes
- [ ] impl UI graph behavior
- [ ] impl UI settings page
- [ ] icons
- [ ] tray icon support (not yet available on [Iced](https://whimsical.com/roadmap-iced-7vhq6R35Lp3TmYH4WeYwLM))
- [ ] theme
- [x] i18n support ([example](https://github.com/pop-os/cosmic-edit/blob/master_jammy/Cargo.toml))

## Repo structure
- [hardware](./hardware/README.md): define an abstraction around the hardware.
- [data](./data/README.md): define structures used in the app (Node, Config), and there logic. Depend on [hardware](./hardware/README.md)
- [ui](./ui/README.md): implement the UI. Depend on [data](./data/README.md) and [hardware](./hardware/README.md)
- the app: integrate all this crates in one executable


# Build
See instructions [here](./BUILD.md).

# Contributing
Contributions are welcome, do not hesitate to open an issue, a pull request, etc...