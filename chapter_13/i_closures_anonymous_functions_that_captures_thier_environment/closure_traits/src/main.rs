#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 7 },
    ];

    // sort_by_key takes a closure that implements Fmut trait
    list.sort_by_key(|r| r.width);
    println!("{list:?}");

    try_sorting_with_fn_once_and_fix();
}

fn try_sorting_with_fn_once_and_fix() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 7 },
    ];

    let mut sort_operations = 0;
    list.sort_by_key(|r| {
        sort_operations += 1;
        r.width
    });
    println!("{list:?}, sorted in {sort_operations} operations.");
}
