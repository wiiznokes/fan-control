## Steps
- [ ] End to End testing
- [ ] package multiple format with [cargo-bundle](https://github.com/burtonageo/cargo-bundle)
    - [ ] Msi
    - [x] Deb
    - [ ] ARM support
    - [ ] RPM (not yet available)
    - [ ] EXE (not yet available)
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
- add cargo-bundle a way to localize ressource
- add macro support for set/get pattern, and to easy pick up of value in a Hashmap / Wrapper-enum
- return error from the windows server
- merge if CI succeed
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