<h1 align="center">Fan Control</h1>

<div>
    <a href="https://flathub.org/apps/io.github.wiiznokes.fan-control"><img align=center height="50" alt="Download on Flathub" src="https://flathub.org/assets/badges/flathub-badge-en.svg"/></a>&nbsp;&nbsp;
    <a href="https://github.com/wiiznokes/fan-control/releases/latest"><img align=center alt="Download on Github release" src="https://img.shields.io/github/release/wiiznokes/fan-control.svg"/></a>&nbsp;&nbsp;
<div>

## Features

- Display sensors data on real time
- Control fans based on custom behaviors
- Save configuration
- Multiplatform (Linux/Windows)

![screenshot of Fan Control](https://media.githubusercontent.com/media/wiiznokes/fan-control/master/res/screenshots/app.png)

## Usage

- You can add items with the buttons on the right of the app.
- To save a configuration, write a name in the "Configuration name" field, and click on the `+`.
- To modify the value of a fan, you must select it in a `Control` item (the left column), select a `Behavior`, and activate the switch.

## Installation

### Windows

1. Install Fan Control from [the release section](https://github.com/wiiznokes/fan-control/releases/latest)

_The configuration files will be in [`C:\Users\wiiz\AppData\Roaming\wiiznokes\fan-control\config`](file:///C:\Users\wiiz\AppData\Roaming\wiiznokes\fan-control\config)._

### Flatpak (Linux)

1. [Install the required udev rules](./res/linux/udev_rules.md)
2. Install fan-control from [Flathub](https://flathub.org/apps/io.github.wiiznokes.fan-control)

_The configuration files will be in [`~/.var/app/io.github.wiiznokes.fan-control/config/fan-control/`](file://~/.var/app/io.github.wiiznokes.fan-control/config/fan-control/)._

<ins>To ensure the application detects the maximum number of sensors, follow these steps</ins>

1. Install `lm-sensors`  
   For Debian-based systems, run: `sudo apt install lm-sensors`  
   For Fedora-based systems, run: `sudo dnf install lm_sensors`
2. Run `sudo sensors-detect` to detect available sensors

## Troubleshooting

See [this file](./TROUBLESHOOTING.md).

## Repo structure

- [hardware](./hardware/README.md): define an abstraction around the hardware.
- [data](./data/README.md): define structures used in the app (Node, Config), and there logic. Depend on [hardware](./hardware/README.md)
- [ui](./ui/README.md): implement the UI. Depend on [data](./data/README.md) and [hardware](./hardware/README.md)
- the app: integrate all this crates in one executable

## Build

See instructions [here](./BUILD.md).

## Contributing

See [this file](./CONTRIBUTING.md).
