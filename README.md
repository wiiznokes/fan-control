<h1 align="center">fan-control</h1>

<div>
    <a href="https://flathub.org/apps/io.github.wiiznokes.fan-control"><img align=center height="50" alt="Download on Flathub" src="https://flathub.org/assets/badges/flathub-badge-en.svg"/></a>&nbsp;&nbsp;
    <a href="https://github.com/wiiznokes/fan-control/releases/latest"><img align=center alt="Download on Github release" src="https://img.shields.io/github/release/wiiznokes/fan-control.svg"/></a>&nbsp;&nbsp;
<div>

## Features

- Display sensors data on real time
- Control fans based on custom behaviors
- Save configuration
- Multiplatform (Linux/Windows)

![screenshot of fan-control](https://media.githubusercontent.com/media/wiiznokes/fan-control/master/resource/screenshots/app.png)

## Usage

- You can add items with the buttons on the right of the app.
- To save a configuration, write a name in the "Configuration name" field, and click on the `+`.
- To modify the value of a fan, you must select it in a `Control` item (the left column), select a `Behavior`, and activate the swtich.

## Installation

<details>
    <summary>Linux</summary>

To have the maximum number of sensors detected by the application, you must

1. install `lm-sensors`:
   - Debian: `sudo apt install lm-sensors`
   - Fedora: `sudo dnf install lm_sensors`
2. run the hardware detection script: `sudo sensors-detect`

For the flatpak version, you need to [install the required udev rules](./resource/linux/udev_rules.md). Then, you can install the app from [flathub](https://flathub.org/apps/io.github.wiiznokes.fan-control).

The configuration file will be in [`~/.var/app/io.github.wiiznokes.fan-control/config/fan-control/`](file://~/.var/app/io.github.wiiznokes.fan-control/config/fan-control/).

</details>

<details>
    <summary>Windows</summary>

The configuration file can be found in [`C:\Users\wiiz\AppData\Roaming\wiiznokes\fan-control\config`](file:///C:\Users\wiiz\AppData\Roaming\wiiznokes\fan-control\config).

</details>

## Repo structure

- [hardware](./hardware/README.md): define an abstraction around the hardware.
- [data](./data/README.md): define structures used in the app (Node, Config), and there logic. Depend on [hardware](./hardware/README.md)
- [ui](./ui/README.md): implement the UI. Depend on [data](./data/README.md) and [hardware](./hardware/README.md)
- the app: integrate all this crates in one executable

## Build

See instructions [here](./BUILD.md).

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md)
