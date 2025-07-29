# Install udev rules on Linux

Modifying fan sensor values is not possible for a normal user by default on Linux. This is why this app uses [udev rules](https://en.wikipedia.org/wiki/Udev), which allows normal users to access devices, without admin privilege.
However, we can't install these rules automatically with Flatpak. You need to install them with these commands.

```sh
wget https://raw.githubusercontent.com/wiiznokes/fan-control/master/res/linux/60-fan-control.rules
sudo mv 60-fan-control.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
```

#### _Information_

- _wget_: download the rule in your current directory
- _sudo mv_: move the rule where it need to be
- _udevadm_: reload the rules

### Steam Os

You need to disable the read only mode temporarily.

```sh
sudo steamos-readonly disable
... commands
sudo steamos-readonly enable
```
