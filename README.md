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

### Flatpak Version

If you're using the Flatpak version of the application, you'll need to [install the required udev rules](./resource/linux/udev_rules.md) first. Then, you can use/install the app from [Flathub](https://flathub.org/apps/io.github.wiiznokes.fan-control).

The configuration file for the Flatpak version will be located at: [`~/.var/app/io.github.wiiznokes.fan-control/config/fan-control/`](file://~/.var/app/io.github.wiiznokes.fan-control/config/fan-control/).

---

<ins>To ensure the application detects the maximum number of sensors, follow these steps:</ins>

1) Install `lm-sensors`  
For Debian-based systems, run: `sudo apt install lm-sensors`  
For Fedora-based systems, run: `sudo dnf install lm_sensors`
2) Run Hardware Detection Script  
After installing `lm-sensors`, execute the following command to detect the available hardware sensors: `sudo sensors-detect`

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

Contributions are welcome, do not hesitate to open an issue, a pull request, etc...
