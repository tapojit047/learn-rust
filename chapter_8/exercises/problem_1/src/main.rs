use std::collections::HashMap;

fn main() {
    let mut vec = vec![10, 2, 2, 2, 3, 30, 50, 100, 70];
    let result1 = problem1(&mut vec);
    println!("Median: {} and Mode: {}", result1.0, result1.1);
}

// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
fn problem1(vec: &mut Vec<i32>) -> (i32, i32) {
    vec.sort();
    let idx_median = vec.len() / 2;
    let median = vec[idx_median];

    let mut map = HashMap::new();
    let mut most_occured = 0;
    for i in vec {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
        if *count > map.get(&most_occured).copied().unwrap_or(0) {
            most_occured = *i;
        }
    }
    (median, most_occured)
}