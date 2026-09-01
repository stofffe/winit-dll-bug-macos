use dlopen::raw::Library;

fn main() {
    let mut hashbrown_hashmap = hashbrown::HashMap::new();
    hashbrown_hashmap.insert('a', 1);
    hashbrown_hashmap.insert('x', 32);

    println!("Call function normally");
    // call function normally (works)
    winit_dll_bug_macos::access_window_functions(&hashbrown_hashmap);

    println!();

    println!("Call function through dll");
    // Dll function (crashes)
    let dll = Library::open("./libwinit_dll_bug_macos.dylib").expect("could not open dll");
    let func =
        unsafe { dll.symbol::<fn(&hashbrown::HashMap<char, u32>)>("access_window_functions") }
            .unwrap();
    func(&hashbrown_hashmap);
}
