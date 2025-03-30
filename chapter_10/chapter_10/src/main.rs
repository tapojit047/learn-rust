fn main() {
    let list1 = vec![10, 200, 30, 40];
    println!("largest: {:?}", largest(&list1));

    let list2 = vec![100, 200, 10000, 40];
    println!("largest: {:?}", largest(&list2));
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}