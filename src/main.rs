#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
    println!("subject: {:?}", subject);
}