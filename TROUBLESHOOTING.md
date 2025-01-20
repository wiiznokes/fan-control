# Windows

The hardware part is provided by [LibreHardwareMonitor](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor). If your hardware isn't detected, you likely need to check if there is an issue open for it in this repo.
The other [FanControl](https://github.com/Rem0o/FanControl.Releases) application also use this library, so you should have the same result with both.

# Linux

You can test the `sensors` program to see detected pwm sensors on your machine. If it is not present here, then it will also not show in the app. You can open an issue in [this repo](https://github.com/lm-sensors/lm-sensors) if this is the case.
In my experience, a lot of laptop fans will not be detected, and NVIDIA is not supported (we need root access for it so no compatible with flatpak).
