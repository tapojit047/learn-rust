use log::debug;

fn main() {
    // we need to annotate Vec<i32>
    // because we are not initializing the vector now
    let vec1: Vec<i32> = Vec::new();

    // Here type annotation not needed
    let vec2 = vec![1, 2, 3];

    // Updating a Vector
    let mut v = Vec::new();

    // The numbers we place inside are all of type i32, and
    // Rust infers this from the data, so we don’t need the Vec<i32> annotation.
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading Elements of Vectors
    let third = &v[2];
    println!("The third element is {}", third);
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let does_not_exist = v[100]; // this will panic the program
    let does_not_exist = v.get(100); // this won't
    match does_not_exist {
        Some(does_not_exist) => println!("The does_not_exist value is {}", does_not_exist),
        None => println!("The value at index 100 does not exist!"),
    }

    // you can’t have mutable and immutable references in the same scope
    /// let first = &v[0];
    v.push(6);

    // now if we access the "first", as it is immutable value
    // the program won't work
    /// println!("The first element is {}", first);

    // Iterating Over the Values in a Vector
    //  how to use a for loop to get immutable references to each element
    for i in &v {
        print!("{i} ");
    }
    println!();

    // mutable reference to each element
    for i in &mut v {
        *i += 50;
        print!("{i} ");
    }
    println!();

    multiple_types()
}


// Using an Enum to Store Multiple Types
fn multiple_types() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        print!("{:?} ", i);
    }
    println!()
}