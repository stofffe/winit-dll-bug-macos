run:
	cargo build
	mv ./target/debug/libhashbrown_dll_bug_macos.dylib .
	cargo run

