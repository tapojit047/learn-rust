mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod customer {
    // this is called idiomatic
    // because we bring in the whole path
    use super::front_of_house::hosting::add_to_waitlist;
    fn trying_out_use() {
        add_to_waitlist();
    }
}

use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// use std::fmt;
// use std::io;
//
// fn function1() -> fmt::Result {
//
// }
// fn function2() -> io::Result<()> {
//
// }


// usage of pub use
mod inner {
    pub struct MyStruct {}

    pub fn my_function() {
        println!("I'm a MyStruct");
    }
}

pub use inner::my_function;

mod outer {
    pub use crate::inner::MyStruct;

    // Re-exporting MyStruct

    // Now MyStruct can be accessed as if it was defined in the outer module
}

use outer::MyStruct;

// Using Nested Paths to Clean Up Large use Lists
/// use std::cmp::Ordering;
/// use std::io;

// If we’re using multiple items defined in the same crate or same module,
// listing each item on its own line can take up a lot of vertical space in our files.
// instead we can do the following:

use std::{cmp::Ordering, io};

// Another example
/// use std::io;
/// use std::io::Write;

// Instead
/// use std::io::{self, Write};