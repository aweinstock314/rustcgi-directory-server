.PHONY: all run

all: target/release/rustcgi-directory-server

run: target/release/rustcgi-directory-server
	./target/release/rustcgi-directory-server

fswatch/autogen.sh:
	(git submodule init && git submodule sync && git submodule update fswatch)

fswatch/libfswatch/src/libfswatch/.libs/libfswatch.a: fswatch/autogen.sh
	(cd fswatch && ./autogen.sh && ./configure && make)

target/release/rustcgi-directory-server: fswatch/libfswatch/src/libfswatch/.libs/libfswatch.a Cargo.toml build.rs src/main.rs
	cargo build --release
