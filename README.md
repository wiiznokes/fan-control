# fan-control

[![license](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](#license)
![ci_status](https://github.com/wiiznokes/fan-control/actions/workflows/test.yml/badge.svg)


# Features
- Display sensors data on real time
- Control fans based on custom behaviors
- Save configuration
- Multiplatform (Linux/Windows)

## Installation
#### Linux
To have the maximum number of sensors detected by the application, you must install lm-sensor and run the hardware detection script:
```
sudo apt install lm-sensors
sudo sensors-detect
```
Also, make sure to execute the program in sudo mode.

## Repo structure
- [hardware](./hardware/README.md): define an abstraction around the hardware.
- [data](./data/README.md): define structures used in the app (Node, Config), and there logic. Depend on [hardware](./hardware/README.md)
- [ui](./ui/README.md): implement the UI. Depend on [data](./data/README.md) and [hardware](./hardware/README.md)
- the app: integrate all this crates in one executable

## Config files
- Windows: `C:\Users\wiiz\AppData\Roaming\wiiznokes\fan-control\config`
- Linux: `/home/wiiz/.config/fan-control`

## Build
See instructions [here](./BUILD.md).

## Contributing
Contributions are welcome, do not hesitate to open an issue, a pull request, etc...
