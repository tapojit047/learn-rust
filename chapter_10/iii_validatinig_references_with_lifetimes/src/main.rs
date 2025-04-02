use std::fmt::Display;
use lifetime_in_struct;

fn main() {
    lifetime_in_funcs();
    // lifetime_in_struct::lifetime_in_struct();
}


fn largest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime_in_funcs() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = largest(string1.as_str(), string2);
    println!("The largest string is {}", result);

    // Let’s look at how the lifetime annotations restrict the longest function
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = largest(string1.as_str(), string2.as_str());
        println!("The largest string is {}", result);
    }

    let result = longest_with_an_announcement(string1.as_str(), string2, "tapojit");
    println!("{result}");
    // This won't work
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = largest(string1.as_str(), string2.as_str());
    // }
    // println!("The largest string is {}", result);
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where
    T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}