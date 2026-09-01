# hashbrown-dll-bug-macos

### Description
This repo is made to replicate the bug in the issue https://github.com/rust-lang/hashbrown/issues/746. 
The code builds a simple DLL and passes hashbrown hashmap over the boundary. The values seem to be kept in the hashmap but lookup using get returns None.

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
