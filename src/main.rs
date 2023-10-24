fn main() {
    let s = String::from("asdfasdf asdfasdf");
    let r = first_word(&s);

    println!("first_word returns {}", &r);
    println!("first_word returns {}", first_word("hello world"));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}