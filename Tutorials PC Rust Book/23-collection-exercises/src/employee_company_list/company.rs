use std::collections::HashMap;

mod department;
mod employee;

pub struct Company {
    departments: HashMap<String, department::Department>
}

impl Company {
    pub fn new() -> Company {
        Company{ departments: HashMap::new() }
    }

    pub fn create_department(&mut self, department_name: &str) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.departments.entry(String::from(department_name)) {
            e.insert(department::Department::new());
        } else {
            println!("Department already exists.");
        }
    }

    pub fn add_employee_to_department(&mut self, employee_name: &str, department_name: &str) {
        match self.departments.get_mut(department_name) {
            Some(department) => department.add_employee(employee_name),
            None => println!("Department does not exist. Create with command 1."), // TODO: change to prompt to create the department and continue
        }
    }

    pub fn get_employees_from_department(&self, department_name: &str) {
        match self.departments.get(department_name) {
            Some(department) => println!("Employees from department {department_name}: {:?}", department.get_employees_sorted()),
            None => println!("Department does not exist. Create with command 1."), // TODO: change to prompt to create the department and continue
        }
    }

    pub fn get_all_employees(&self) {
        for department in self.departments.keys() {
            self.get_employees_from_department(department);
        }
    }
}

#[test]
fn test_create_department() {
    let mut company = Company::new();

    company.create_department("IT");
    company.create_department("HR");

    assert_eq!(company.departments.len(), 2);
    assert!(company.departments.contains_key("IT"));
    assert!(company.departments.contains_key("HR"));
}

#[test]
fn add_employee_to_department() {
    let mut company = Company::new();

    company.create_department("IT");
    company.add_employee_to_department("employee", "IT");

    let department = company.departments.get("IT").unwrap();
    let employees = department.get_employees();

    assert_eq!(employees.len(), 1);
    assert!(employees.contains(&String::from("employee")));
}

#[test]
fn add_employee_to_invalid_department() {
    let mut company = Company::new();

    company.add_employee_to_department("employee", "IT");

    assert_eq!(company.departments.len(), 0);
}

#[test]
fn get_employees_from_department() {
    let mut company = Company::new();

    company.create_department("IT");
    company.add_employee_to_department("employee1", "IT");
    company.add_employee_to_department("employee2", "IT");
    company.add_employee_to_department("employee3", "IT");
    let mut employees = company.departments.get("IT").unwrap().get_employees();
    employees.sort();

    assert_eq!(employees.len(), 3);
    let mut employees = employees.into_iter();
    assert_eq!(employees.next().unwrap(), "employee1");
    assert_eq!(employees.next().unwrap(), "employee2");
    assert_eq!(employees.next().unwrap(), "employee3");

}

