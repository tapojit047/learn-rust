use std::io;

fn main() {
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    // Declaring tuples
    let tup: (i32, f64, u8) = (-500, 6.3, 1);

    let tup = (1, 2, "Tapojit", "Paul");

    // destructuring a tuple
    let (a, b, firstName, lastName) = tup;
    println!("The fullname is {firstName} {lastName}!!");

    // accessing individual values of a tuple with indices
    let firstValue = tup.0;
    let secondValue = tup.1;
    println!("{firstValue}, {secondValue}");

    arrays();
    playWithArray();
}


fn arrays() {
    // arrays are fixed in length
    let ara = [1, 2, 3, 4, 5];
    let first = ara[0];
    let second = ara[1];
    println!("{} XXXXXXXXXXXX {} XXXXXXXXXXXX {}", first, second, ara[2]);

    // initalizing array with the same value
    let ara = [3; 5];
    println!("{} XXXXXXXXXXXX {} XXXXXXXXXXXX {}", ara[0], ara[1], ara[2]);

    // using explicit type to declare the array
    let ara: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{} {}", ara[0], ara[1]);
}

fn playWithArray() {
    let mut ara = [1, 2, 4, 5, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = ara[index];

    println!("The value of the element at index {index} is: {element}");
}