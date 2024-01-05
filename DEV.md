## Steps

- [ ] End to End testing
- [ ] package multiple format with [cargo-packager](https://github.com/crabnebula-dev/cargo-packager)
  - [ ] Msi
  - [x] Deb
  - [ ] ARM support
  - [ ] RPM (not yet available)
  - [ ] EXE (not yet available)
  - [x] NSIS
  - [ ] Flatpak (not yet available)
  - [ ] Snap (not yet available)
  - [x] CI for packaging
- [ ] impl UI graph behavior
- [ ] tray icon support (not yet available on [Iced](https://whimsical.com/roadmap-iced-7vhq6R35Lp3TmYH4WeYwLM))

## Missing piece from Iced/Libcosmic (Alpha software)

- Text ellapcing
- rendering multiple layers
- windows icons (close/maximize)

## Todo/upgradable parts

- add cargo-packager a way to localize ressource: done but not merged
- add app icon on windows, with cargo packager (nsis)
- add resource folder only for nsis
- return error from the windows server
- stop windows server with a command: implemented but could be improved with taking app state back
- get app_state back when we close the ui
- pop up when closing
- serialize graph like that:
  ```toml
  [[Graph]]
  name = "Graph"
  input = "max"
  cood = [
      { temp = 50, percent = 30 },
      { temp = 50, percent = 30 },
      { temp = 50, percent = 30 },
  ]
  ```
- make a similar crate of https://github.com/mxre/winres, but with no dependencies. This will add an icon to .exe on Windows https://github.com/crabnebula-dev/cargo-packager/issues/107
- find out how to do blocking operation on the command pool of Iced, without blocking the thread
- support const value in trait that use enum_dispatch
- fix the time_stamp of env_logger
- pop when leaving the app, to save the current config or auto create temp config

## run specific test:

```
clear && cargo test --package hardware test_time -- --nocapture
```
