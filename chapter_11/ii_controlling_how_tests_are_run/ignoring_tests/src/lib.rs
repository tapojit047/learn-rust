pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore] // It specifies to ignore this tests
    fn expensive_test() {
        // code that takes one hour to run
    }
}


// $ cargo test
// this will ignore tests with #[ignore] tag

// $ cargo test -- --ignored
// this will run only the tests with the #[ignore] tag

// $ cargo test -- --include-ignored
// this will run all the tests