use std::thread;

fn main() {
    // closure_basics();
    // closure_capturing_immutable_referencing();
    // closure_capturing_mutable_referencing();
    closure_in_thread();
}

fn closure_basics() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);  --> Cannot do this, type of x is already inferred as "String"
    println!("{}", s);

    let add_one_v4 = |x| x + 1;
    let x_1 = add_one_v4(100);
    println!("Result of adding 1: {}", x_1);
}

fn closure_capturing_immutable_referencing() {
    let list = vec![1, 2, 3];
    // Closure takes Immutable Reference
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

fn closure_capturing_mutable_referencing() {
    let mut list = vec![1, 2, 3];
    // Closure takes Immutable Reference
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(4);

    // println!("Before calling closure: {list:?}");
    // Cannot do this, because
    // Between the closure definition and the closure call,
    // an immutable borrow to print isn’t allowed because no other borrows
    // are allowed when there’s a mutable borrow.

    borrows_mutably();
    println!("After calling closure: {list:?}");
}

fn closure_in_thread() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();
}