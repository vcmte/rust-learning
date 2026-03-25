mod front_of_house;

pub use front_of_house::hosting;
/* usage of front_house::hosting is out of scope in module customer, so we can't call that
mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
*/

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

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

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    /* // The code below won't compile cuz of private fields in Breakfast struct
    let mut meal2 = back_of_house::Breakfast {
        toast: String::from("Wheat"),
        seasonal_fruit: String::from("blueberries"),
    }
    */

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

    // ####################################################

    // All fields of public enums are public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // ####################################################

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // ####################################################

    // Namespace usage (through use) made this possible
    hosting::add_to_waitlist();

    // ####################################################
}
