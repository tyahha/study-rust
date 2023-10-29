pub fn collection_main() {
    let v: Vec<i32> = Vec::new();
    println!("v: {:?}", v);

    let v = vec![1, 2, 3];
    println!("v: {:?}", v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v: {:?}", v);

    let third = &v[2];

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    // v.push(6);

    println!("The third element is {}", third);

    for i in &mut v {
        *i += 50;
    }
    println!("v: {:?}", v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("row: {:?}", row);
}
