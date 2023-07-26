mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn sear_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
    
    fn fix_incorrect_order () {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {

    }
}

// Absolute path
// use crate::front_of_house::hosting;
// Related path
// use front_of_house::hosting;

// re-exporting
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // reative path
    front_of_house::hosting::add_to_waitlist();

    /* struct */
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // let mut meal = back_of_house::Breakfast { 
    //     toast: String::from("Rye"), 
    //     seasonal_fruit: String::from("peaches") 
    // }; // error
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast", meal.toast);

    /* enum */
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    /* use */
    hosting::add_to_waitlist();

    /* re-exporting */
    hosting::add_to_waitlist();

}

use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> Result {
    
// }

// fn function2() -> IoResult<()> {

// }