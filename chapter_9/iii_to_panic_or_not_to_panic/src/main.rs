use std::io;

fn main() {
    use_guess();
}


// https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation
// Creating Custom Types for Validation
// We can use Guess as the parameter to make sure that a function
// only take integer input in the range 1 to 100
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

// we are going to take input of number
// and make sure that the number is within 1 to 100
fn use_guess() {
    loop {
        println!("HIIIIIIIIIIIIIIIIII");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let num: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", num);
        let guess = Guess::new(num);
    }
}