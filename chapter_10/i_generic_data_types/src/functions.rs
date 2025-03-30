pub fn funcs_main() {
    let number_list = vec![10, 200, 30, 40];
    println!("largest: {:?}", largest_i32(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("largest: {:?}", largest_char(&char_list));
}


fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

