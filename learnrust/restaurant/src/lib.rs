#[allow(dead_code)]
mod front_of_house {
    // "src/front_of_house.rs" <-- code inside {} should be in this file
    // alternatively "src/front_of_house/mod.rs" <-- old style
    // lib.rs should only have mod front_of_house;

    // In rust, all items are private to parent modules by default.
    // Items in a parent module can't use private items inside child
    // modules but items in child modules can use the items in their
    // ancestor modules.
    pub mod hosting {
        // Making the module public doesn't make its contents public
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

#[allow(dead_code)]
mod back_of_house {
    fn test() {
        // It's possible to construct relative paths that begin in
        // parent module by using 'super' akin to '../' in a filesystem
        super::eat_at_restaurant();
    }

    // A pub struct's fields will still be private, fields
    // have to individually be made private as per need.
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

    // All variants of a public enum are public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// Adding 'use' keyword creates a shortcut to a path, akin to
// creating a symbolic link in the filesystem.
pub use crate::front_of_house::hosting;
// ↑ 'pub use' makes the new imported name brought into scope as public otherwise its private
// its useful as it allows use to write code with one structure while exposing a different one

// if below is used, external code would call add_to_waitlist() by
// using full path and would require mod front_of_house to be 'pub'
// use crate::front_of_house::hosting; // <--
// to extract hosting to its own file: "src/front_of_house/hosting.rs"

#[allow(unused_imports)]
pub fn eat_at_restaurant() {
    // Since eat_at_restaurant and front_of_house share modules
    // (siblings) it can refer to it even though it's not public.
    crate::front_of_house::hosting::add_to_waitlist(); // absolute path
    front_of_house::hosting::add_to_waitlist(); // relative path

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    // hosting is brought into scope where 'use' occurs
    // hence if eat_at_restaurant was wrapped in another module, it won't work
    // specifying parent module when calling fn makes its source clear
    // but when bringing structs, enums etc with use, its idiomatic to specify full path
    hosting::add_to_waitlist();

    #[allow(dead_code)]
    mod use_wont_work {
        // use crate::hosting; // <-- fixes the below
        pub fn hosting_not_in_scope() {
            // hosting::add_to_waitlist(); // <-- failed to resolve
            super::hosting::add_to_waitlist(); // werks
        }
    }

    use ::std::collections::*; // glob * brings all public items from path into scope

    // Bringing two types with same name can be resolved either by
    // importing till their parent module or by aliasing them using 'as'
    use std::{fmt::Result, io::Result as IoResult}; // <-- nested paths
}

//* Boilerplate added by cargo
// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
