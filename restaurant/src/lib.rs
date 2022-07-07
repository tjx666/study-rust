#![allow(dead_code)]
#![allow(unused_variables)]

pub mod front_of_house;

pub mod back_of_house;

pub fn eat_at_rest() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I would like {} toast please", meal.toast);

    // field `seasonal_fruit` of struct `Breakfast` is private
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
}
