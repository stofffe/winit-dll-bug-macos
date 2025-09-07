run:
	cargo build
	mv ./target/debug/libwinit_dll_bug_macos.dylib .
	cargo run

