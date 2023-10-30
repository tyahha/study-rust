use std::collections::HashMap;

pub fn hash_map_main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    scores.insert(String::from("Blue"), 50);

    println!("scores: {:?}", scores);

    let teams = vec![
        String::from("Blue"),
        String::from("Yellow"),
        String::from("Red"),
    ];
    let initial_scores = vec![10, 60];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores: {:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map: {:?}", map);

    // error[E0382]: borrow of moved value: `field_value`
    // println!("{}", field_value);

    println!("Blue team score: {:?}", scores.get(&String::from("Blue")));

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
