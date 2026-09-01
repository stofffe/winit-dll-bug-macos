#[unsafe(no_mangle)]
pub fn access_window_functions(hashmap: &hashbrown::HashMap<char, u32>) {
    println!("HashMap = {:?}", hashmap);
    println!("HashMap lookup of a = {:?}", hashmap.get(&'a'));
    println!("HashMap lookup of x = {:?}", hashmap.get(&'x'));
}
