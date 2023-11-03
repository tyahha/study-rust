mod collection;
mod error;
mod lifetime;
mod test_sample;
mod trait_sample;

fn main() {
    println!("--- 8. Common collections");
    collection::collection_main();

    println!("--- 9. Error handling");
    error::error_main(false);

    println!("--- 10.2 Trait");
    trait_sample::trait_main();

    println!("--- 10.3 lifetime");
    lifetime::lifetime_main();
}

#[cfg(test)]
mod test {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("panic");
    }
}
