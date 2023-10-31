


conf:
	clear && cargo run -- -p ./config

release:
	clear && cargo run --release

run:
	clear && cargo run

fix:
	cargo clippy --all --fix --allow-dirty --allow-staged
	cargo fmt --all

clean-git:
	git rm -rf --cached .
	git add .

expand:
	clear && cargo expand


linux:
	# git clone --depth=1 --branch=pwm https://github.com/wiiznokes/libsensors.git
	$(MAKE) -C ./libsensors install PREFIX=./../lib ETCDIR=./../etc
	cd sensors-sys
	export LMSENSORS_STATIC=1
	export LMSENSORS_INCLUDE_DIR=./../lib/include
	export LMSENSORS_LIB_DIR=./../lib/lib
	cargo build

clean:
	$(MAKE) -C ./libsensors uninstall clean PREFIX=./../lib ETCDIR=./../etc