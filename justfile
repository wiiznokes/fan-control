set windows-powershell := true

all:
	cargo build --release
	
## Build Libs
	
libsensors:
	git submodule update --init hardware/libsensors
	make -C ./hardware/libsensors/ install PREFIX=./../../build/libsensors ETCDIR=./../../build/libsensors/etc

lhm:
	dotnet build ./hardware/LibreHardwareMonitorWrapper/ -c Release

## Packaging

package-deb:
	cargo packager --release --formats deb
	mkdir -p packages
	cp ./target/release/fan-control*.deb ./packages/

package-nsis:
	cargo packager --release --formats nsis
	mkdir -p packages
	cp ./target/release/fan-control*-setup.exe ./packages/

## Test

fix:
	cargo clippy --all --fix --allow-dirty --allow-staged
	cargo fmt --all

fix-lhm:
	dotnet format ./hardware/LibreHardwareMonitorWrapper/LibreHardwareMonitorWrapper.csproj

test:
	cargo test --all --all-features

## Clean

clean-libsensors:
	make -C ./hardware/libsensors/ clean uninstall PREFIX=./../../build/libsensors ETCDIR=./../../build/libsensors/etc

clean-lhm:
	dotnet clean ./hardware/LibreHardwareMonitorWrapper/



## Handy

fake:
	cargo run --features fake_hardware -- -p ./.config -c fake

temp:
	cargo run --features fake_hardware -- -p ./temp

git-cache:
	git rm -rf --cached .
	git add .

run-lhm:
	dotnet run --project ./hardware/LibreHardwareMonitorWrapper/ -c Release

expand:
	cargo expand




## Debug

debll:
	dpkg-deb -c ./packages/fan-control*.deb | grep -v usr/lib/fan-control/icons/

debi: package-deb debll
	sudo apt-get remove fan-control -y || true > /dev/null
	sudo apt-get install ./packages/fan-control_0.1.0_amd64.deb > /dev/null
	fan-control

debinfo:
	dpkg-query -s fan-control

debl:
	dpkg-query -L fan-control | grep -v /usr/lib/fan-control/ressource/

package-msi:
	cargo bundle --release --format msi

