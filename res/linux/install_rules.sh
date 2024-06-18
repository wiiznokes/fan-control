#!/bin/bash


# see https://openrgb.org/udev.html

STEAMOS=0
STEAMOS_READONLY=0

# Test for SteamOS and disable readonly mode if we're running on it
if command -v steamos-readonly >& /dev/null
then
	# Test if SteamOS readonly mode is enabled
	if sudo steamos-readonly status | grep 'enabled'
	then
		echo "steamos readonly mode is true"
		STEAMOS_READONLY=1
	fi

	STEAMOS=1
	sudo steamos-readonly disable
fi

sudo groupadd -f fan-control
CURRENT_USER=$(whoami)
sudo usermod -aG fan-control $CURRENT_USER

wget https://raw.githubusercontent.com/wiiznokes/fan-control/master/res/linux/60-fan-control.rules -P /tmp/ > /dev/null
sudo mv /tmp/60-fan-control.rules /etc/udev/rules.d/

sudo udevadm control --reload-rules
sudo udevadm trigger

if [ "$STEAMOS" = 1 ] ; then
	if [ "$STEAMOS_READONLY" = 1 ] ; then
		sudo steamos-readonly enable
	fi
fi