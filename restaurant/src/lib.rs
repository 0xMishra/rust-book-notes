mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    // struct also have to use pub keyword even the fields of a struct
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // seasonal_fruit is private we have to use impl block to create a new instance of Breakfast and add the pub keyword in function summer as well
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }

    pub enum Appetizer {
        // enum variants are public by default
        Soup,
        Salad,
    }
}

pub use crate::front_of_house::hosting; //simplifying import now we don't have to write the entire absolute or relative path everytime
pub fn eat_at_restaurant() {
    // ways to access a function in the same crate
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    // change the bread
    meal.toast = String::from("wheat");
    println!("I would like {} toast please", meal.toast);
    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
}
