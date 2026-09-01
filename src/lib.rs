#[unsafe(no_mangle)]
pub fn use_hashbrown_hashmap(
    std_hashmap: &std::collections::HashMap<char, u32>,
    hashbrown_hashmap: &hashbrown::HashMap<char, u32>,
) {
    println!("std HashMap = {:?}", std_hashmap);
    println!("std HashMap lookup of a = {:?}", std_hashmap.get(&'a'));
    println!("std HashMap lookup of x = {:?}", std_hashmap.get(&'x'));

    println!();

    println!("hashbrown HashMap = {:?}", hashbrown_hashmap);
    println!(
        "hashbrown HashMap lookup of a = {:?}",
        hashbrown_hashmap.get(&'a')
    );
    println!(
        "hashbrown HashMap lookup of x = {:?}",
        hashbrown_hashmap.get(&'x')
    );
}
