use dlopen::raw::Library;

fn main() {
    let mut hashbrown_hashmap = hashbrown::HashMap::new();
    hashbrown_hashmap.insert('a', 1);
    hashbrown_hashmap.insert('x', 32);

    let mut std_hashmap = std::collections::HashMap::new();
    std_hashmap.insert('a', 1);
    std_hashmap.insert('x', 32);

    println!("--- Call function normally ---");
    println!();
    // call function normally (works)
    hashbrown_dll_bug_macos::use_hashbrown_hashmap(&std_hashmap, &hashbrown_hashmap);

    println!();

    println!("--- Call function through dll ---");
    println!();
    // Dll function (crashes)
    let dll = Library::open("./libhashbrown_dll_bug_macos.dylib").expect("could not open dll");
    let func = unsafe {
        dll.symbol::<fn(&std::collections::HashMap<char, u32>, &hashbrown::HashMap<char, u32>)>(
            "use_hashbrown_hashmap",
        )
    }
    .unwrap();
    func(&std_hashmap, &hashbrown_hashmap);
}
