mod front_of_house;

pub mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    use back_of_house::{Appetizer, Breakfast as Hoge};
    use front_of_house::hosting;

    hosting::add_to_watitlist();

    let mut meal = Hoge::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
    println!("{:?}, {:?}", order1, order2);
}
