pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_wishlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute
    crate::front_of_house::hosting::add_to_wishlist();

    // relative
    front_of_house::hosting::add_to_wishlist();
}

// super
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    // publish structure
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
}

pub fn eat_breakfast() {
    let mut meal = back_of_house::Breakfast::summer("yamazaki");

    meal.toast = String::from("meiji");
    println!("breakfast toast is {}", meal.toast);

    // 下記はpubじゃないのでダメ
    // meal.seasonal_fruits = String::from("apples");
}

// divide file
mod child_mod;

use child_mod::grandchild_mod;

pub fn check() {
    child_mod::show();
    grandchild_mod::show();
}
