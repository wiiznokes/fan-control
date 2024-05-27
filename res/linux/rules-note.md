# Some notes on udev rules

### see some logs:

```sh
journalctl -f
```

### reload

```sh
sudo udevadm control --reload-rules && sudo udevadm trigger
```

### show info about triggering and devices (test the hwmon7 device ?)

```sh
sudo udevadm test /sys/class/hwmon/hwmon7
```

### show info

```sh
udevadm monitor -u | grep hwmon
```

## Other

```sh
ll /sys/class/hwmon/hwmon7/pwm*
sudo /bin/chmod a-w /sys/class/hwmon/hwmon7/pwm*
cat /tmp/fan-control-udev.txt
wget https://openrgb.org/releases/release_0.9/60-openrgb.rules
```

- `TAG+="uaccess"` is not enough on my PC.
- `chmod g+w` is not enough on my PC.
- only the hwmon7 seems to be relevant on my PC.
- hwmon7 is the kernel name. It can be refered as %k in the rules file.
- %p is the device path. For example, the device path of hwmon7 (`/sys/class/hwmon/hwmon7`) is `/sys/devices/platform/nct6775.656/hwmon/hwmon7`

`ACTION=="add"`: the rule will only be triggered when a device is added to the system
`KERNEL=="hwmon[0-99]"`: match any version of hwmon kernel name

### Export log to a file in a RUN clause

```
RUN+="/bin/sh -c '{COMMAND} > /tmp/fan-control-udev.txt 2>&1'"
```

`chmod a+w /sys%p/pwm*`: The \* will only make sense in a shell program, that why we spawn /bin/sh before

### Attempt to filter pwmX and pwmX_enable

This two files are the only one that the app use.
The regex and command work in a terminal on my machine, but don't work inside a RUN clause.

```
SUBSYSTEM=="hwmon", KERNEL=="hwmon7", RUN+="/bin/sh -c '/bin/find /sys%p -type f -regex '/sys%p/pwm[0-99]\(_enable\)?' -exec /bin/chmod a+w {} \; > /tmp/fan-control-udev.txt 2>&1'"
```

### path for rules:

- /etc/udev/rules.d/60-fan-control.rules
- /usr/lib/udev/rules.d/60-fan-control.rules

```
# LOGS VERSION
# SUBSYSTEM=="hwmon", KERNEL=="hwmon7", RUN+="/bin/sh -c '/bin/chmod -v a+w /sys%p/pwm* > /tmp/fan-control-udev.txt 2>&1'"
```
