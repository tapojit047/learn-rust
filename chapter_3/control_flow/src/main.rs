fn main() {
    let number = 6;
    if is_odd(6) {
        println!("{number} is odd");
    } else {
        println!("{number} is even");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // infinite_loop();
    // breaking_loop();
    // labeling_loop();
    // while_loop();
    // while_looping_array_elements();
    // for_looping_array_elements();
    for_loop();
}

fn is_odd(number: i32) -> bool {
    if number % 2 == 0 {
        return false;
    } else {
        return true;
    }
}

fn infinite_loop() {
    loop {
        println!("again!");
    }
}

fn breaking_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn labeling_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
        println!("----------------");
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
}

fn while_looping_array_elements() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("{index}th value is: {}", a[index]);
        
        index += 1;
    }
}

fn for_looping_array_elements() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("value is: {element}");
    }
}

fn for_loop() {
    for number in (1..10).rev() {
        println!("{number}!");
    }
}