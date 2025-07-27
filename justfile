set windows-powershell := true

export FAN_CONTROL_VERSION := "2025.3.0"
export FAN_CONTROL_COMMIT := `git rev-parse --short HEAD`
rootdir := ''
prefix := '/usr'
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
    cargo build {{ args }}

build-release *args:
    cargo build --release {{ args }}

libsensors:
    git submodule update --init hardware/libsensors
    just build-libsensors

build-libsensors:
    make -C ./hardware/libsensors/ install PREFIX=./../../build/libsensors ETCDIR=./../../build/libsensors/etc

lhm:
    dotnet build ./hardware/LibreHardwareMonitorWrapper/ -c Release

install:
    install -Dm0755 {{ bin-src }} {{ bin-dst }}
    install -Dm0644 res/linux/desktop_entry.desktop {{ desktop-dst }}
    install -Dm0644 res/linux/metainfo.xml {{ metainfo-dst }}
    install -Dm0644 res/linux/app_icon.svg {{ icon-dst }}

uninstall:
    rm -f {{ bin-dst }}
    rm -f {{ desktop-dst }}
    rm -f {{ metainfo-dst }}
    rm -f {{ icon-dst }}

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

###################  Flatpak

runf:
    RUST_LOG="warn,fan-control=debug" flatpak run {{ appid }}

uninstallf:
    flatpak uninstall {{ appid }} -y || true

update-flatpak: setup-update-flatpak update-flatpak-gen commit-update-flatpak

# deps: flatpak-builder git-lfs
build-and-installf: uninstallf
    flatpak-builder \
        --force-clean \
        --verbose \
        --user \
        --install \
        --install-deps-from=flathub \
        --repo=repo \
        flatpak-out \
        {{ repo-name }}/{{ appid }}.json

sdk-version := "24.08"

install-sdk:
    flatpak remote-add --if-not-exists --user flathub https://flathub.org/repo/flathub.flatpakrepo
    flatpak install --noninteractive --user flathub \
        org.freedesktop.Platform//{{ sdk-version }} \
        org.freedesktop.Sdk//{{ sdk-version }} \
        org.freedesktop.Sdk.Extension.rust-stable//{{ sdk-version }} \
        org.freedesktop.Sdk.Extension.llvm18//{{ sdk-version }}

repo-name := "flatpak-repo"

# pip install aiohttp toml
setup-update-flatpak:
    rm -rf {{ repo-name }}
    git clone https://github.com/wiiznokes/io.github.wiiznokes.fan-control.git {{ repo-name }}
    git -C {{ repo-name }} remote add upstream https://github.com/flathub/io.github.wiiznokes.fan-control.git
    git -C {{ repo-name }} fetch upstream
    git -C {{ repo-name }} checkout master
    git -C {{ repo-name }} rebase upstream/master master
    git -C {{ repo-name }} push origin master

    git -C {{ repo-name }} branch -D update-{{ name }} || true
    git -C {{ repo-name }} push origin --delete update-{{ name }} || true
    git -C {{ repo-name }} checkout -b update-{{ name }}
    git -C {{ repo-name }} push origin update-{{ name }}

    rm -rf flatpak-builder-tools
    git clone https://github.com/flatpak/flatpak-builder-tools

update-flatpak-gen:
    python3 flatpak-builder-tools/cargo/flatpak-cargo-generator.py Cargo.lock -o {{ repo-name }}/cargo-sources.json
    cp flatpak_schema.json {{ repo-name }}/{{ appid }}.json
    sed -i "s/###commit###/$(git rev-parse HEAD)/g" {{ repo-name }}/{{ appid }}.json

commit-update-flatpak:
    git -C {{ repo-name }} add .
    git -C {{ repo-name }} commit -m "Update {{ name }}"
    git -C {{ repo-name }} push origin update-{{ name }}
    xdg-open https://github.com/flathub/io.github.wiiznokes.fan-control/compare/master...wiiznokes:update-{{ name }}?expand=1

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
