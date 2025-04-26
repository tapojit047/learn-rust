use std::collections::HashSet;

fn main() {
    // iterator_basics();
    // iterator_sum();
    // iterator_adapter_methods();

    let v1 = vec![1, 2, 3];
    let v2 = trying_things_out(&v1, 10);
    println!("{:?}", v2);
}

fn iterator_basics() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
}

fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    // After we call methods like sum() on a iterator, we can not reuse the iterator.
    // Because that function has used the iterator to calculate the sum
    assert_eq!(total, 6);
    println!("Total: {}", total);
}

fn iterator_adapter_methods() {
    let v1 = vec![1, 2, 3, 30, 30];
    let mut v1_iter = v1.iter().map(|x|  x + 10);

    while true {
        let op_val = v1_iter.next();
        match op_val {
            Some(val) => println!("Got: {}", val),
            None => break,
        }
    }
    println!();

    // we can also directly collect a new resultant vector from the iterator
    let v2: Vec<_> = v1.iter().map(|x| x + 100).collect();
    println!("{:?}\n", v2);

    // Why we need to explicitly mention Vector (Vec<_>)?
    // Because when you call .collect(), Rust needs to know what type of collection you want to collect into.
    // .collect() can collect into many types — like a Vec, a HashSet, a String, etc.
    // Let's try to collect to a HashSet
    let hash_set: HashSet<_> = v1.iter().map(|x| x + 100).collect();
    println!("{:?}\n", hash_set);
}

fn trying_things_out(v1: &Vec<i32>, size: u32) -> Vec<i32> {
    v1.iter().map(|x| x+100).collect()
}