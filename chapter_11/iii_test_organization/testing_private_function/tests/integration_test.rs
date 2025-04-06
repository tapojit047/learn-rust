use testing_private_function::add_two;
mod common;

#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}

// $ cargo test --> this will run all unit, integration, and doc test

// $ cargo test --test integration_test
// To run all the tests in a particular integration test file,
// use the --test argument of cargo test followed by the name of the file