mod front_of_house;

mod serving {
    fn take_order() {}
    fn take_payment() {}
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order(); // You can call cook order as it exists inside back_of_hour
        super::serve_order();
        // We can use super to call serve_order(), using relative path and parent module
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::add_to_waitlist();
}
