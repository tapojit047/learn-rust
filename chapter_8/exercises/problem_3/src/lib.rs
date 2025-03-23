pub mod listing {
    use std::collections::HashMap;

    pub fn handle_list_by_department(list: Option<&mut Vec<String>>, department: &String) {
        print!("List of employees of the {department}: ");
        match list {
            Some(employee_list) => {
                employee_list.sort();
                for employee in employee_list {
                    print!("{} ", employee);
                }
                println!();
            }
            None => {
                println!("No employees found for {department}");
            }
        }
    }

    pub fn handle_list_all(map: &mut HashMap<String, Vec<String>>) {
        for (department, employees) in map {
            print!("\nList of employees in {department}: ");
            for employee in employees {
                print!("{} ", employee);
            }
        }
        println!();
    }
}
