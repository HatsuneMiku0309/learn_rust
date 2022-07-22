mod front_of_house {
    mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house;

pub fn test() {
    front_of_house::hosting::add_to_waitlist();
}
