#[derive(Debug)]
#[derive(PartialEq)]
struct AlwaysEqual;
#[derive(Debug)]
#[derive(PartialEq)]
struct AlwaysEqual2;

fn main() {
    let subject1 = AlwaysEqual;
    let subject2 = AlwaysEqual;
    let subject3 = AlwaysEqual2;
    println!("subject1: {:?}", subject1);
    println!("subject2: {:?}", subject2);
    println!("subject3: {:?}", subject3);
    println!("subject1 == subject2: {}", subject1 == subject2);
    // println!("subject1 == subject3: {}", subject1 == subject3); // compile error
}