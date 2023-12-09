
fake:
	cargo run --features fake_hardware -- -p ./.config -c fake

## Build Libs
	
libsensors:
	git submodule update --init hardware/libsensors
	make -C ./hardware/libsensors/ install PREFIX=./../../target/libsensors_build ETCDIR=./../../target/libsensors_build/etc

lhm:
	dotnet build ./hardware/LibreHardwareMonitorWrapper/ -c Release

## Packaging

package-deb:
	cargo bundle --release --format deb 

## Test

fix:
	cargo clippy --all --fix --allow-dirty --allow-staged
	cargo fmt --all

fix-lhm:
	dotnet format ./hardware/LibreHardwareMonitorWrapper/LibreHardwareMonitorWrapper.csproj

test:
	cargo test --all --all-features

## Clean

clean:
	cargo clean

clean-libsensors:
	make -C ./hardware/libsensors/ clean uninstall PREFIX=./../../target/libsensors_build ETCDIR=./../../target/libsensors_build/etc

clean-lhm:
	dotnet clean ./hardware/LibreHardwareMonitorWrapper/





.PHONY: clean-libsensors libsensors




# Temp

temp:
	clear && cargo run --features fake_hardware -- -p ./.test -c test

run-lhm:
	dotnet run --project ./hardware/LibreHardwareMonitorWrapper/ -c Release


git-cache:
	git rm -rf --cached .
	git add .


expand:
	cargo expand


conf:
	cargo run -- -p ./.config

deb: package-deb
	sudo apt remove fan-control -y
	sudo apt install ./target/release/bundle/deb/fan-control_0.1.0_amd64.deb
	dpkg-deb -c ./target/release/bundle/deb/fan-control_0.1.0_amd64.deb

deb-i:
	dpkg-query -s fan-control

deb-l:
	dpkg-query -L fan-control