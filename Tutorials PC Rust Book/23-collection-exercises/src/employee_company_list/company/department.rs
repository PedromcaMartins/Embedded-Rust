// use super::employee;

use std::collections::HashSet;

pub struct Department {
    employees: HashSet<String>
}

impl Department {
    pub fn new() -> Department {
        Department { employees: HashSet::new()}
    }

    pub fn add_employee(&mut self, name: &str) {
        if !self.employees.insert(String::from(name)) {
            println!("Employee is already part of the department");
        }
    }

    pub fn get_employees(&self) -> Vec<String> {
        Vec::from_iter(self.employees.clone())
    }

    pub fn get_employees_sorted(&self) -> Vec<String> {
        let mut employees = self.get_employees();
        employees.sort();
        employees
    }
}

#[test]
fn test_add_employee() {
    let mut department = Department::new();
    department.add_employee("employee");

    assert!(department.employees.contains("employee"));
}

#[test]
fn test_add_employee_duplicate() {
    let mut department = Department::new();
    department.add_employee("employee");
    department.add_employee("employee");

    assert_eq!(department.employees.len(), 1);
}

#[test]
fn test_get_employees() {
    let mut department = Department::new();
    department.add_employee("employee1");
    department.add_employee("employee2");
    department.add_employee("employee3");

    let employees: Vec<String> = department.get_employees();

    assert_eq!(employees.len(), 3);
    assert!(employees.contains(&String::from("employee1")));
    assert!(employees.contains(&String::from("employee2")));
    assert!(employees.contains(&String::from("employee3")));
}

#[test]
fn test_get_employees_sorted() {
    let mut department = Department::new();
    department.add_employee("employee1");
    department.add_employee("employee2");
    department.add_employee("employee3");

    let employees: Vec<String> = department.get_employees_sorted();

    assert_eq!(employees.len(), 3);
    let mut employees = employees.into_iter();
    assert_eq!(employees.next().unwrap(), "employee1");
    assert_eq!(employees.next().unwrap(), "employee2");
    assert_eq!(employees.next().unwrap(), "employee3");
}