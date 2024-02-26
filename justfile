set windows-powershell := true

all:
	cargo run --release

# call before pull request
pull: fmt prettier fix test
	
###################  Build Libs

libsensors:
	git submodule update --init hardware/libsensors
	make -C ./hardware/libsensors/ install PREFIX=./../../build/libsensors ETCDIR=./../../build/libsensors/etc

lhm:
	dotnet build ./hardware/LibreHardwareMonitorWrapper/ -c Release

###################  Packaging

deb:
	cargo packager --release --formats deb --verbose
	mkdir -p packages
	cp ./target/release/fan-control*.deb ./packages/

nsis:
	cargo packager --release --formats nsis --verbose
	New-Item -Path .\packages -ItemType Directory -Force > $null
	cp ./target/release/fan-control*-setup.exe ./packages/

###################  Test

test:
	cargo test --workspace --all-features

###################  Format

fix:
	cargo clippy --workspace --all-features --fix --allow-dirty --allow-staged

fmt:
	cargo fmt --all

prettier:
	# install on Debian: sudo snap install node --classic
	# npx is the command to run npm package, node is the runtime
	npx prettier -w .
	
fmt-lhm:
	dotnet format ./hardware/LibreHardwareMonitorWrapper/LibreHardwareMonitorWrapper.csproj

###################  Clean

clean-libsensors:
	make -C ./hardware/libsensors/ clean uninstall PREFIX=./../../build/libsensors ETCDIR=./../../build/libsensors/etc

clean-lhm:
	dotnet clean ./hardware/LibreHardwareMonitorWrapper/










###################  Handy

fake:
	cargo run --features fake_hardware -- -p ./.config
	

temp:
	cargo run --features fake_hardware -- -p ./temp

conf:
	cargo run -- -p ./.config

git-cache:
	git rm -rf --cached .
	git add .

run-lhm:
	dotnet run --project ./hardware/LibreHardwareMonitorWrapper/ -c Release --log=info

expand:
	cargo expand




## Debug

debll:
	dpkg-deb -c ./packages/fan-control*.deb | grep -v usr/lib/fan-control/icons/

debi: deb debll
	sudo apt-get remove fan-control -y || true > /dev/null
	sudo apt-get install ./packages/fan-control_0.1.0_amd64.deb > /dev/null
	fan-control

debinfo:
	dpkg-query -s fan-control

debl:
	dpkg-query -L fan-control | grep -v /usr/lib/fan-control/ressource/

package-msi:
	cargo bundle --release --format msi


flatpak-sdk:
	flatpak remote-add --if-not-exists --user flathub https://flathub.org/repo/flathub.flatpakrepo
	flatpak install --noninteractive --user flathub org.freedesktop.Platform//22.08 org.freedesktop.Sdk//22.08 org.freedesktop.Sdk.Extension.rust-stable//22.08


flatpak:
	# sudo apt install flatpak-builder
	flatpak-builder \
		--force-clean \
		packages \
		resource/flatpak/com.wiiznokes.fan-control.json