# Install udev rules on Linux

## Info
fan-control is distributed with flatpak on Linux, which allows containerization. But fan-control need to access your sensors to work.
This is why we use [udev](https://en.wikipedia.org/wiki/Udev), which allows normal user to access devices, without giving them sudo permission. [This is](./60-fan-control.rules) the rule that you need for fan-control.

# Commands to install the rules

```sh
wget https://raw.githubusercontent.com/wiiznokes/fan-control/resource/linux/60-fan-control.rules
sudo mv 60-fan-control.rules /usr/lib/udev/rules.d
sudo udevadm control --reload-rules && sudo udevadm trigger
```

#### *Info*
- *wget*: download the rule in your current directory
- *sudo mv*: move the rule where it need to be
- *udevadm*: reload the rules

### Steam Os

You need to disable read the read only mode temporarily

```sh
sudo steamos-readonly disable
... commands
sudo steamos-readonly enable
```