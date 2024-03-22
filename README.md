# Features

- Display sensors data on real time
- Control fans based on custom behaviors
- Save configuration
- Multiplatform (Linux/Windows)

![fan-control](https://github.com/wiiznokes/fan-control/assets/78230769/cdc30753-4186-47a1-ba49-11af3868380f)

# Usage

- You can add items with the buttons on the right of the app.
- To save a configuration, write a name in the "Configuration name" field, and click on the `+`.
- To modify the value of a fan, you must select it in a `Control` item (the left column), select a `Behavior`, and activate the swtich.

# Installation

<details>
    <summary>Linux</summary>

To have the maximum number of sensors detected by the application, you must

1. install `lm-sensors`:
   - Debian: `sudo apt install lm-sensors`
   - Fedora: `sudo dnf install lm_sensors`
2. run the hardware detection script: `sudo sensors-detect`

Also, make sure to run the application with sudo. (running in user mode is [planned](https://wiki.archlinux.org/title/udev))

The configuration file will be in [`~/.config/fan-control`](file://~/.config/fan-control) or [`/root/.config/fan-control`](file:///root/.config/fan-control).

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
