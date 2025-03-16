use std::io;

fn main() {
    println!("Enter a sentence: ");
    let mut text = String::new();
    io::stdin().read_line(&mut text).unwrap();

    text = problem_2(text);
    print!("{}", text);
}

// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay.
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
fn problem_2(text: String) -> String {
    let mut result = String::new();
    for word in text.split_whitespace() {
        let mut chars = word.chars();
        if let Some(first_char) = chars.next() {
            // let first_char = &word[..1];
            if result.len() > 0 {
                result.push(' ');
            }
            if is_vowel(first_char) {
                result.push_str(&format!("{word}-hay"));
            } else {
                result.push_str(&word[1..]);
                result.push_str(&format!("-{first_char}ay"));
            }
        }
    }
    result
}
fn is_vowel(ch: char) -> bool {
    ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' ||
        ch == 'A' || ch == 'E' || ch == 'I' || ch == 'O' || ch == 'U'
}