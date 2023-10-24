fn main() {
    let s = String::from("asdfasdf asdfasdf");
    let r = first_word(&s);

    println!("first_word returns {}", &r);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}