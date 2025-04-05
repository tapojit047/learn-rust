struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }
        Guess { value }
    }

    fn new_with_bug(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // should_panic checks whether the panic message contains
    // the string we have put in the expected as a substring
    // which helps us to write test specified for a particular error
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn check_error_precisely() {
        Guess::new(200);
    }

    // For example, there is a bug in the function new_with_bug.
    // We will use the should_panic to detect that the error exists.
    // Even though the code will panic, there is an issue with the logic.
    // This test will help us detect that bug
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn detect_bug() {
        Guess::new_with_bug(200);
    }
}
