fn main() {
    let v = Some(3);

    if let Some(3) = v {
        println!("three");
    }

    if let Some(5) = v {
        println!("file");
    } else {
        println!("no five");
    }
}