mod hash_map;
mod string;
mod vector;

pub fn collection_main() {
    println!("  ----- vector");
    vector::vector_main();

    println!("  ----- string");
    string::string_main();

    println!("  ----- hash map");
    hash_map::hash_map_main();
}
