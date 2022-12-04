use std::io::*;
use std::collections::HashMap;
fn main() {
    println!("Welcome to the Company's employee system!");
    let mut employees: HashMap<String, String> = HashMap::new();
    loop {
        let mut decision = String::new();
        println!("1) Add employee");
        println!("2) See employees in a department");
        println!("3) See employees in the entire company");
        println!("4) Exit");

        stdin().read_line(&mut decision)
                .expect("Failed to read the line.");
        
        let decision: i32 = match decision.trim().parse() {
            Ok(num) => num,
            Err(_) => { 
                println!("This is not a number!");
                continue
            }
        };
        match decision {
            1 => {
                let new_employee = add_employee();
                let current_employee = employees.entry(new_employee.0).or_insert("".to_string());
                *current_employee += &new_employee.1; 
            },
            2 => {
                see_department_employees(&employees);
            },
            3 =>{ 
                see_all_employees(&employees);
            },
            4 => {
                println!("See you later!");
                break;
            },
            _ => println!("Try with a valid number.")
        }
    }
}

//Function to add employees
fn add_employee() -> (String,String){
    let mut name = String::new();
    let mut department = String::new();
    println!("Welcome!");
    println!("Who do you want to add?");
    stdin().read_line(&mut name)
           .expect("This is not a valid string");

    println!("In what department do the work in?");
    stdin().read_line(&mut department)
           .expect("This is not a valid string!");
    
    (department,name)
}

//See all the people in a deparment
fn see_department_employees(deparment: &HashMap<String, String>) {
    let mut department_name = String::new();
    print!("What is the deparment you want to see into? ");
    stdin().read_line(&mut department_name)
           .expect("This string failed");

    match deparment.get(&department_name) {
        Some(val) => println!("{val}"),
        None => (),
    }
}

//See all the people in the company
fn see_all_employees(employees: &HashMap<String, String>) {
    if employees.is_empty() {
        println!("There are not employees in this company");
    } else {
        println!("Here are all employees!");
        for (_,v) in employees.iter() {
            println!("{v}");
        }
    }
}