use std::fmt::{write, Display, Formatter, Pointer, Write};

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("default")
    }
}

pub struct NewsArticles {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticles {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Hoge {}
impl Summary for Hoge {}
impl Display for Hoge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("hoge")
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary, U: Summary>(item1: &T, item2: &U) {
    println!(
        "Breaking news2! {}, {}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn notify3<T: Summary + Display>(item: &T) {
    println!("summary: {}, value: {}", item.summarize(), item);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn trait_main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let hoge = Hoge {};
    println!("Hoge, {}", hoge.summarize());

    notify(&tweet);
    notify(&hoge);
    notify2(&tweet, &hoge);
    notify3(&hoge);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    let result = largest2(&char_list);
    println!("The largest char is {}", result);
}
