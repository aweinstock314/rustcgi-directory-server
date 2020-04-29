.PHONY: all run

all: target/release/rustcgi-directory-server

run: target/release/rustcgi-directory-server
	./target/release/rustcgi-directory-server

fswatch:
	git submodule update fswatch

fswatch/libfswatch/src/libfswatch/.libs/libfswatch.a: fswatch
	(cd fswatch && ./autogen.sh && ./configure && make)

target/release/rustcgi-directory-server: Cargo.toml build.rs src/main.rs
	cargo build --release
