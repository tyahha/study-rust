mod collection;
mod error;
mod lifetime;
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
