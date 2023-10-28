mod front_of_house {
    pub mod hosting {
        pub fn add_to_watitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_watitlist();

    front_of_house::hosting::add_to_watitlist();
}
