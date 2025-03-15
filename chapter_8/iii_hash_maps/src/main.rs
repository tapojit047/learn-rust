use std::collections::HashMap;

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("team score is {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    hash_map_and_ownership();

    // overwriting a particular value of a key
    scores.insert(String::from("Blue"), 20);
    println!("{:?}", scores); // "Blue": 20, the value is updated

    // if we want to update only if the value for that particular key does not exist,
    // then, we can use entry
    scores.entry(String::from("Orange")).or_insert(10); // this will insert a new key-value pair ("Orange": 10)
    scores.entry(String::from("Blue")).or_insert(10); // this won't insert a new value, because the value already exists

    println!("{:?}", scores);

    count_words(String::from("hello hello hello how low"));
}

fn hash_map_and_ownership() {
    let mut map = HashMap::new();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    map.insert(field_name, field_value);

    // println!("{field_name}, {field_value}");
    // We cannot do this because string has a MOVE trait
    // So the ownership of the values are moved from
    // field_name and field_value to the hashMap
}

fn count_words(text: String) {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}