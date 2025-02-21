#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enum type can have associated functions as well
impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    let n = Message::Quit;
    n.call();

    let o = Message::Move { x: 3, y: 4 };
    o.call();

    // Programming language design is often thought of in terms of which features you include,
    // but the features you exclude are important too. Rust doesn’t have the null feature
    // that many other languages have. Null is a value that means there is no value there.
    // In languages with null, variables can always be in one of two states: null or not-null.

    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    // it is defined by the standard library
    // it is used when we want to define a variable
    // that we want to have null value

    let some_number = Some(5);
    let some_char = Some('t');

    let absent_number: Option<i32> = None;
    // For absent_number, Rust requires us to annotate the overall Option type:
    // the compiler can’t infer the type that the corresponding
    // Some variant will hold by looking only at a None value.
    // Here, we tell Rust that we mean for absent_number to be of type Option<i32>.

    println!("some_number: {:?}", some_number);
    println!("absent_number: {:?}", absent_number);
}
