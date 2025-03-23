use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(file) => file,
                    Err(e) => panic!("Problem creating the file: {e:?}"),
                },
                other_error => {
                    panic!("Problem opening the file: {error:?}")
                }
            }
        }
    };

    // Using unwrap function which handles the error i.e. calls the panic! in case of failure for us
    // instead of writing the descriptive code above
    let greeting_file = File::open("hello.txt").unwrap();

    // Using expect function which handles the error for us i.e. calls the panic!
    // in case of failure for us with a custom message we have provided
    // instead of writing the descriptive code above
    let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");


    // Lets try propagating error
    let result = read_username_from_file();
    match result {
        Ok(username) => println!("The username is {username}"),
        Err(e) => panic!("There was a problem opening the file: {e:?}"),
    }

    let result = read_username_from_file_concise();
    match result {
        Ok(username) => println!("The username is {username}"),
        Err(e) => panic!("There was a problem opening the file: {e:?}"),
    }

    let result = read_username_from_file_standard_way();
    match result {
        Ok(username) => println!("The username is {username}"),
        Err(e) => panic!("There was a problem opening the file: {e:?}"),
    }

    let ch = last_char_of_the_first_line("\nhello world");
    match ch {
        Some(c) => println!("The current char of the ch is {c}"),
        None => println!("The current char of the ch doesn't exist"),
    }
}

use std::io::{self, Read};
// Propagating Errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Here "?" operator returns the error automatically
// without needing us to handle the error manually
fn read_username_from_file_concise() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; // Here if an error occurs, it is returned immediately by "?"
    Ok(username)
}

fn read_username_from_file_more_concise() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

use std::fs;
fn read_username_from_file_standard_way() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_the_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}