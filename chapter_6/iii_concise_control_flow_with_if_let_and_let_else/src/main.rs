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

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            _ => false,
        }
    }
}

fn describe_state_quarter(coin: &Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn describe_state_quarter_2(coin: &Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

// let - else syntax
fn describe_state_quarter_3(coin: &Coin) -> Option<String> {
    // Here state is exported to outer scope automatically
    // when coin is quarter
    // otherwise only else is executed and returned immediately
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}


fn main() {
    let config_max = Some(255u8);
    match config_max {
        Some(max) => println!("The maximum number is {}", max),
        _ => (),
    }

    // This is requiring us to write boilerplate code
    // Using if let means less typing, less indentation, and less boilerplate code.
    // However, you lose the exhaustive checking that match enforces.
    // Choosing between match and if let depends on what you’re doing in your particular situation
    // and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.
    // In other words, you can think of if let as syntax sugar for a match that runs code when
    // the value matches one pattern and then ignores all other values.
    if let Some(max) = config_max {
        println!("The maximum number is {}", max);
    }

    // ---------------------------------------------------------------------------------------------

    let coin = Coin::Quarter(UsState::Alabama);
    let answer = describe_state_quarter(&coin);
    println!("The answer is {}", answer.unwrap());

    let answer2 = describe_state_quarter_2(&coin);
    println!("{}", answer2.unwrap());


}
