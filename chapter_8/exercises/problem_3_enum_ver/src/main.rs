use std::collections::HashMap;
use std::io;
use problem_3::listing;

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

// Add --> defines the command to add a new employee to a particular department
// List <String (Department) > --> defines the command to list all the employees of a department
// List <None> --> defines the command to list all the employees by department
enum Command {
    Add(String, String),
    List(Option<String>),
}

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    println!("Provide Command in the 3 following formats: ");
    println!("1. Add <Employee-Name> to <Department> --> To add an employee to a particular department.");
    println!("2. List <Department> --> To list all the employee of a department.");
    println!("3. List --> List all the employees of the company by department");
    println!("------------------------------------------------------------------------------------------");
    loop {
        println!("Provide command: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = parse_command(input);
        exec_command(command, &mut map);
        println!();
    }
}

// exec_command received the command as Command enum
// and then executes the functionality of that command
fn exec_command(command: Option<Command>, map: &mut HashMap<String, Vec<String>> ) {
    match command {
        Some(Command::Add(name, department)) => {
            let list = map.entry(department.clone()).or_insert(Vec::new());
            list.push(name.clone());

            println!("{name} is successfully added to the {department}");
        }
        Some(Command::List(Some(department))) => {
            let list = map.get_mut(&department);
            listing::handle_list_by_department(list, &department);
        }
        Some(Command::List(None)) => {
            listing::handle_list_all(map);
        }
        None => {
            println!("Invalid Command!!!");
        }
    }
}

// parse_command parses the input command and
// maps and returns the command enums
fn parse_command(input: String) -> Option<Command> {
    let mut words = input.split_whitespace();
    let mut name = String::new();
    let mut department = String::new();
    let mut command: Option<Command> = None;
    while let Some(word) = words.next() {
        if word == "Add" {
            if let Some(next_word) = words.next() {
                name = next_word.to_string();
            }
        }

        else if word == "to" {
            if let Some(next_word) = words.next() {
                department = next_word.to_string();
            }
            command = Option::from(Command::Add(name.clone(), department.clone()));
        }

        else if word == "List" {
            // List <Department>
            if let Some(next_word) = words.next() {
                department = next_word.to_string();
                command = Option::from(Command::List(Option::from(department.clone())));
            } else {
                // List --> List All]
                command = Option::from(Command::List(Option::from(None)));
            }
        }
        else {
            println!("Invalid Command!!!");
        }
    }
    command
}

// Input:
// Add Tapojit to Engineering
// Add Matin to Engineering
// Add Salik to Management
// Add Zahid to Management
// Add Tamal to Engineering
// Add Tamal to Management
// List Engineering
// List Management
// List Manage
// List