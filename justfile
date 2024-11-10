set windows-powershell := true

export FAN_CONTROL_VERSION := '2024.11'
export FAN_CONTROL_COMMIT := `git rev-parse --short HEAD`

rootdir := ''
prefix := x"~/.local"
debug := '0'

name := 'fan-control'
appid := 'io.github.wiiznokes.' + name

cargo-target-dir := env('CARGO_TARGET_DIR', 'target')
bin-src := cargo-target-dir / if debug == '1' { 'debug' / name } else { 'release' / name }

base-dir := absolute_path(clean(rootdir / prefix))
share-dst := base-dir / 'share'

bin-dst := base-dir / 'bin' / name
desktop-dst := share-dst / 'applications' / appid + '.desktop'
metainfo-dst := share-dst / 'metainfo' / appid + '.metainfo.xml'
icon-dst := share-dst / 'icons/hicolor/scalable/apps' / appid + '.svg'


default: build-release

# call before pull request
pull: fmt prettier fix test

build-debug *args:
	cargo build {{args}}

build-release *args:
  cargo build --release {{args}}
	

libsensors:
	git submodule update --init hardware/libsensors
	make -C ./hardware/libsensors/ install PREFIX=./../../build/libsensors ETCDIR=./../../build/libsensors/etc

lhm:
	dotnet build ./hardware/LibreHardwareMonitorWrapper/ -c Release


install: 
	install -Dm0755 {{bin-src}} {{bin-dst}}
	install -Dm0644 res/linux/desktop_entry.desktop {{desktop-dst}}
	install -Dm0644 res/linux/metainfo.xml {{metainfo-dst}}
	install -Dm0644 res/linux/app_icon.svg {{icon-dst}}


uninstall:
	rm {{bin-dst}}
	rm {{desktop-dst}}
	rm {{metainfo-dst}}
	rm {{icon-dst}}
	
###################  Packaging

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

# todo: add to CI when ubuntu-image get appstream version 1.0
metainfo-check:
	appstreamcli validate --pedantic --explain --strict res/linux/metainfo.xml
	
fmt-lhm:
	dotnet format ./hardware/LibreHardwareMonitorWrapper/LibreHardwareMonitorWrapper.csproj

###################  Clean

clean-libsensors:
	make -C ./hardware/libsensors clean || true
	rm -rf build/libsensors || true
	
clean-lhm:
	dotnet clean ./hardware/LibreHardwareMonitorWrapper/ || true










###################  Handy

fake:
	cargo run --features fake_hardware -- -p ./configs-examples
	

temp:
	cargo run --features fake_hardware -- -p ./temp

conf:
	cargo run -- -p ./configs-examples

git-cache:
	git rm -rf --cached .
	git add .

run-lhm:
	dotnet run --project ./hardware/LibreHardwareMonitorWrapper/ -c Release --log=info

expand:
	cargo expand
