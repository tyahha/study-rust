pub fn string_main() {
    let s = String::from("あいうえお");

    println!("length of 'あいうえお': {}", s.len());

    // error[E0277]: the type `String` cannot be indexed by `{integer}`
    // println!("first index access of 'あいうえお': {}", &s[0])

    for c in s.chars() {
        println!("{}", c);
    }

    for c in s.bytes() {
        println!("{}", c);
    }
}
