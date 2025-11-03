use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Takes a string slice and return string literal
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {

    // THIS IS USING THE BOX of standard library
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // NOW WE WILL USE A CUSTOM SMART POINTER TYPE MyBox
    let mb = MyBox::new(x);
    assert_eq!(5, *mb);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // this call is equivalent to and rust does it internally
    hello(&(*m)[..])
    // first calls the deref() on m and gets the reference to
}
