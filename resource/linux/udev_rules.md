# Install udev rules on Linux

## Info

fan-control is distributed with flatpak on Linux, which allows containerization. But fan-control need to access your sensors to work.
This is why we use [udev](https://en.wikipedia.org/wiki/Udev), which allows normal user to access devices, without giving them sudo permission. [This is](./60-fan-control.rules) the rule that you need for fan-control.

# Commands to install the rules

```sh
wget https://raw.githubusercontent.com/wiiznokes/fan-control/master/resource/linux/60-fan-control.rules
sudo mv 60-fan-control.rules /usr/lib/udev/rules.d
sudo udevadm control --reload-rules && sudo udevadm trigger
```

#### _Info_

- _wget_: download the rule in your current directory
- _sudo mv_: move the rule where it need to be
- _udevadm_: reload the rules

### Steam Os

You need to disable the read only mode temporarily

```sh
sudo steamos-readonly disable
... commands
sudo steamos-readonly enable
```
