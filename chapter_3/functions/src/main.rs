fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("High {}!!!", x);

    let x = plus_one(x);
    println!("x became {}", x);
}

// 1. Rust uses snake case as the 
//    conventional style for functions and variable names
// 2. we could have defined it before as well. 
//    Rust doesn’t care where you define your functions,
fn another_function(x: i32) {
    println!("The value of parameter x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// returing a value from a function
// the returning line should be an expression (which yields a value)
// expressions do not have semicolon at the end
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}