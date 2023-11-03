
run:
	cargo run

conf:
	cargo run -- -p ./config

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