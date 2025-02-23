#[derive(Debug)]
enum UsState {
    Texas,
    Georgia,
    California,
    Virginia,
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// Ref: https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont
// https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont
// Patterns That Bind to Values
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

// Ref: https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont
// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// the arms’ patterns must cover all possibilities
// Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid.
fn main() {
    let penny = Coin::Penny;
    println!("{}", value_in_cents(penny));

    let dime = Coin::Dime;
    println!("{}", value_in_cents(dime));

    let nickel = Coin::Nickel;
    println!("{}", value_in_cents(nickel));

    let tx_quarter = Coin::Quarter(UsState::Texas);
    println!("{}", value_in_cents(tx_quarter));

    let ga_quarter = Coin::Quarter(UsState::Georgia);
    println!("{}", value_in_cents(ga_quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{six:?} {none:?}");

    // Catch-all Patterns
    let mut hat = 0;
    hat = roll_dice(3, hat);
    hat = roll_dice(7, hat);
    hat = roll_dice(7, hat);
    hat = roll_dice(9, hat);
}

fn roll_dice(dice_roll: u8, mut hat: u32) -> u32 {
    match dice_roll {
        3 => hat = add_fancy_hat(hat),
        7 => hat = remove_fancy_hat(hat),
        other => move_player(other),
    }
    hat
}

fn add_fancy_hat(mut hat: u32) -> u32 {
    println!("Current hat count: {hat}. Adding a fancy hat!");
    hat + 1
}

fn remove_fancy_hat(mut hat: u32) -> u32 {
    println!("Current hat count: {hat}. Removing fancy hat!");
    if hat <= 0 {
        println!("Oops! No hat found to remove.");
        return hat
    }
    hat - 1
}

fn move_player(num_spaces: u8) {
    println!("Moving player {num_spaces} places");
}
fn reroll() {
    println!("No action defined. Roll again...")
}