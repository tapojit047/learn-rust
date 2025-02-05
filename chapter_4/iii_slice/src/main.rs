fn main() {
    let s = String::from("Hello World!");

    let hello = &s[0..5];
    let world = &s[7..12];

    println!("{} {}", hello, world);

    let word = first_word(&s);

    // s.clear(); // Error because we are using a reference to s later

    println!("{word}");

    let word = first_word(&s[0..3]);
    let word = first_word(&s[1..3]);

    let word = first_word(&s);

    let string_literal = "hello world";
    let word = first_word(string_literal);
    let word = first_word(&string_literal[1..5]);
}

// Find the first word in a string
// 'string slice' is written as '&str'
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}