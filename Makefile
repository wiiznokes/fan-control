
run:
	clear && cargo run

fake:
	clear && cargo run --features fake_hardware -- -p ./.config

conf:
	clear && cargo run -- -p ./.config

config:
	clear && ./target/debug/fan-control -p ./config

release:
	clear && cargo run --release

fix:
	cargo clippy --all --fix --allow-dirty --allow-staged
	cargo fmt --all

clean-git:
	git rm -rf --cached .
	git add .

expand:
	clear && cargo expand

libsensors:
	make -C ./libsensors/ install PREFIX=./../libsensors_build ETCDIR=./../etc

clean-libsensors:
	make -C ./libsensors/ clean uninstall PREFIX=./../libsensors_build ETCDIR=./../etc

test:
	clear && cargo test --all --all-features


.PHONY: clean-libsensors libsensors