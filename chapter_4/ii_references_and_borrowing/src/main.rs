fn main() {
    let mut s = String::from("Hello");
    let len = calculate_length(&s);
    println!("The length of '{s}' is {len}.");

    change(&mut s);
    println!("{s}");

    // Multiple Mut variable
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}", r1);
    // We can not do this:
        // Borrow multiple times
        // Then use the use the previously borrowed reference

    // Can not do this either:    
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;
    // println!("{} {}", r1, r2);

    // However, multiple immutable references are allowed
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", World!");
}

// It is not possible to declare this function
// Because reference is deallocated and returned simultanously
// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s
// }

// This one is correct
fn no_dangle() -> String {
    let s = String::from("Hello");
    s
}