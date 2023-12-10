## Env

package-deb: export PACKAGE_TYPE=DEB

## Build Libs
	
libsensors:
	git submodule update --init hardware/libsensors
	make -C ./hardware/libsensors/ install PREFIX=./../../build/libsensors ETCDIR=./../../build/libsensors/etc

lhm:
	dotnet build ./hardware/LibreHardwareMonitorWrapper/ -c Release

## Packaging

package-deb:
	cargo bundle --release --format deb
	mkdir -p bundle
	cp ./target/release/bundle/deb/fan-control*.deb ./bundle/

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



.PHONY: clean-libsensors libsensors




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

debi: package-deb
	sudo apt remove fan-control -y || true
	sudo apt install ./target/release/bundle/deb/fan-control_0.1.0_amd64.deb
	# dpkg-deb -c ./target/release/bundle/deb/fan-control_0.1.0_amd64.deb
	fan-control
debinfo:
	dpkg-query -s fan-control

debl:
	dpkg-query -L fan-control

package-msi:
	cargo bundle --release --format msi 