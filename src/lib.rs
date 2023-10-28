pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_watitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {
            take_payment();
            super::hosting::add_to_watitlist();
        }
        fn serve_order() {}
        fn take_payment() {}
    }
}

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
    use back_of_house::Appetizer;
    use back_of_house::Breakfast as Hoge;
    use front_of_house::hosting;

    hosting::add_to_watitlist();

    let mut meal = Hoge::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
    println!("{:?}, {:?}", order1, order2);
}
