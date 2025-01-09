use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    // Generates a secret number which the user will try to guess
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count = 1;

    loop {
        println!("Guess Number: {}", count);
        println!("Please input your guess.");
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number!!!!\n");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
        }
        count = count + 1;
    }
    println!("Took {count} guesses to win.");

}
