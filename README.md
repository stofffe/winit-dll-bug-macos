# winit-dll-bug-macos

### Description
This repo is made to replicate the bug in the issue https://github.com/rust-windowing/winit/issues/4344. 
The code builds a simple DLL which accesses the winit Window. The DLL is then loaded and called, leading to a crash.

### How to replicate
First build the library DLL
```
cargo build
```
Then move the DLL from target to root
```
mv ./target/debug/libwinit_dll_bug_macos.dylib .
```
Then run the program
```
cargo run
```

This can also be ran using a Makefile with
```
make
```
