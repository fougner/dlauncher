PREFIX?=/usr/local
VERSION?=$(shell git describe --always --dirty=.dirty 2>/dev/null || echo v0.1-nogit)

run_debug:
	RUST_LOG=debug cargo run

run_x11:
	GDK_BACKEND=x11 RUST_LOG=debug cargo run

run:
	./target/release/dlauncher

run_debug_log:
	RUST_LOG=debug ./target/release/dlauncher

build:
	DEV_VERSION=-$(VERSION) cargo build --release

docs:
	cargo doc --no-deps

install:
	install -D -o root -g root -m 755 ./target/release/dlauncher "${PREFIX}/bin/dlauncher"
	install -D -o root -g root -m 755 ./target/release/dlauncher-toggle "${PREFIX}/bin/dlauncher-toggle"