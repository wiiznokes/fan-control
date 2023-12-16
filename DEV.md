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
- add cargo-packager a way to localize ressource
- add app icon on windows (nsis)
- add resource folder only for nsis
- execute as admin by default: nsis
- add macro support for set/get pattern, and to easy pick up of value in a Hashmap / Wrapper-enum
- return error from the windows server
- stop windows server with a command: implemented but could be improved with taking app state back
- use thiserror or anyhow
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
- make a similar crate of https://github.com/mxre/winres, but with no dependencies. This will add an icon to .exe on Windows