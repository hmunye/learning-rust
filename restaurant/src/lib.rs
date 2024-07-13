mod restaurant {
    pub mod front {
        pub fn add_to_list() {}

        pub fn seat_table() {}
    }

    mod back {
        fn take_order() {}

        fn serve() {}

        fn payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute Path
    crate::restaurant::front::add_to_list();

    // Relative Path
    restaurant::front::seat_table();
}
