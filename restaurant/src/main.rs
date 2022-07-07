use restaurant::{back_of_house::fix_incorrect_order, front_of_house::hosting};

pub mod main_mod {
    pub fn hello() {
        println!("hello");
    }
}

fn main() {
    hosting::add_to_waitlist();
    main_mod::hello();
    fix_incorrect_order();
}
