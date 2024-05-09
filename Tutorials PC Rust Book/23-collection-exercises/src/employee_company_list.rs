use std::io;

mod company;

pub fn start_console() {
    let mut company = company::Company::new();
    println!("Welcome to the Company Portal for Employee Management.\nHere are the available commands: ");

    loop {
        println!();
        println!("Create Department: 1 <Department Name>");
        println!("Add Employee to Department: 2 <Employee Name> <Department Name>");
        println!("Get Employees from Department: 3 <Department Name>");
        println!("Get All Employees: 4");
        println!("Quit: 5");

        loop {
            println!();
            println!("Please, input the command you want to use");

            let mut input = String::new();
            if let Err(error) = io::stdin().read_line(&mut input) { 
                println!("error reading from stdin: {error}")
            }

            input = String::from(input.trim_end());
            let mut iter = input.split_ascii_whitespace();
            let command = iter.next().unwrap_or("-1").parse::<i32>().unwrap_or(-1);

            match command {
                1 => match iter.next() {
                    Some(department_name) => { 
                        company.create_department(department_name); 
                        break; 
                    },
                    None => println!("Invalid Command. Expected: 1 <Department_name>"),
                },
                2 => match (iter.next(), iter.next()) {
                    (Some(employee_name), Some(department_name)) => { 
                        company.add_employee_to_department(employee_name, department_name); 
                        break; 
                    },
                    (_, _) => println!("Invalid Command. Expected: 2 <Employee_name> <Department_name>"),
                },
                3 => match iter.next() {
                    Some(department_name) => { 
                        company.get_employees_from_department(department_name); 
                        break; 
                    },
                    None => println!("Invalid Command. Expected: 3 <Department_name>"),
                },
                4 => {
                    company.get_all_employees();
                    break;
                },
                5 => return,
                _ => println!("Invalid Command."),
            }
        }
    }
}