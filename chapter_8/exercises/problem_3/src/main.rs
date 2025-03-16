use std::collections::HashMap;
use std::io;

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    println!("Provide Command in the 3 following formats: ");
    println!("1. Add <Employee-Name> to <Department> --> To add an employee to a particular department.");
    println!("2. List <Department> --> To list all the employee of a department.");
    println!("3. List --> List all the employees of the company by department");
    println!("-------------------------------------------------------------------------------------------------");
    loop {
        println!("Provide command: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        handle_input(input, &mut map);

        println!("");
    }
}

fn handle_input(input: String, map: &mut HashMap<String, Vec<String>> ) {
    let mut words = input.split_whitespace();
    let mut name = String::new();
    let mut department = String::new();

    while let Some(word) = words.next() {
        if word == "Add" {
            if let Some(next_word) = words.next() {
                name = next_word.to_string();
            }
        }

        if word == "to" {
            if let Some(next_word) = words.next() {
                department = next_word.to_string();
            }
            println!("{name} is successfully added to the {department}");
        }

        if word == "List" {
            // List <Department>
            if let Some(next_word) = words.next() {
                department = next_word.to_string();
                let list = map.get_mut(&department);

                handle_list_by_department(list, &department);
            } else {
                // List --> List All]
                handle_list_all(map);
            }
        }

    }
    let list = map.entry(department).or_insert(Vec::new());
    list.push(name);
}

fn handle_list_by_department(list: Option<&mut Vec<String>>, department: &String) {
    print!("List of employees of the {department}: ");
    match list {
        Some(employee_list) => {
            employee_list.sort();
            for employee in employee_list {
                print!("{} ", employee);
            }
            println!("");
        }
        None => {
            println!("No employees found for {department}");
        }
    }
}

fn handle_list_all(map: &mut HashMap<String, Vec<String>>) {
    for (department, employees) in map {
        print!("\nList of employees in {department}: ");
        for employee in employees {
            print!("{} ", employee);
        }
    }
    println!();
}



// Input:
// Add Matin to Engineering
// Add Tapojit to Engineering
// Add Salik to Management
// Add Zahid to Management
// Add Tamal to Engineering
// Add Tamal to Management
// List Engineering
// List Management
// List Manage
// List