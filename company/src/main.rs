extern crate generational_arena;
extern crate linked_hash_set;

use generational_arena::Arena;
use linked_hash_set::LinkedHashSet;
use std::io::{self, Write};

//TODO: save/load to/from json

fn getchar() {
    //TODO: not a real getchar(), but this seems hard to do :(
    //      ideally what i'd want is to wait for some specific keystroke (Space, Enter, maybe a few select others), without echoing anything, but still consuming everything
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
}

enum Prompt {
    Value(String),
    Empty,
    Eof,
}

fn prompt(text: &str) -> Prompt {
    print!("{}", text);
    io::stdout().flush().unwrap();

    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("FAILED TO READ LINE");
    if text.is_empty() {
        return Prompt::Eof;
    }
    let text = text.trim().to_string();
    if text.is_empty() {
        Prompt::Empty
    } else {
        Prompt::Value(text)
    }
}

enum Choice<'a, T> {
    Entry(&'a str, T, generational_arena::Index),
    List(&'a str, T),
    New(&'a str, T),
    Add(&'a str, T),
    Move(&'a str, T),
    Remove(&'a str, T),
    Cancel(&'a str),
}

//TODO: store action to perform in (optional) lambda?
fn choose<T>(choices: Vec<Choice<T>>) -> Choice<T> {
    for (i, c) in choices.iter().enumerate() {
        let text = match c {
            Choice::Entry(t, _, _) => t.to_string(),
            Choice::List(t, _) if t.is_empty() => "[LIST]".to_string(),
            Choice::List(t, _) => format!("[LIST {}]", t),
            Choice::New(t, _) if t.is_empty() => "[NEW]".to_string(),
            Choice::New(t, _) => format!("[NEW {}]", t),
            Choice::Add(t, _) if t.is_empty() => "[ADD]".to_string(),
            Choice::Add(t, _) => format!("[ADD {}]", t),
            Choice::Move(t, _) if t.is_empty() => "[MOVE]".to_string(),
            Choice::Move(t, _) => format!("[MOVE {}]", t),
            Choice::Remove(t, _) if t.is_empty() => "[REMOVE]".to_string(),
            Choice::Remove(t, _) => format!("[REMOVE {}]", t),
            Choice::Cancel(t) => format!("[{}]", t),
        };
        println!("{}) {}", i + 1, text);
    }
    println!();
    loop {
        match prompt("CHOICE? ") {
            Prompt::Value(choice) => {
                match choice.parse::<usize>() {
                    Ok(num) => {
                        if 0 < num && num < choices.len() + 1 {
                            // extract and return a single Choice from the vector
                            break choices.into_iter().nth(num - 1).unwrap();
                        } else {
                            continue;
                        }
                    }
                    Err(_) => continue,
                };
            }
            Prompt::Empty => {
                continue;
            }
            Prompt::Eof => {
                break Choice::Cancel("");
            }
        }
    }
}

struct DataStore {
    companies: Arena<Company>,
    departments: Arena<Department>,
    employees: Arena<Employee>,
}

impl DataStore {
    fn new() -> DataStore {
        DataStore {
            companies: Arena::new(),
            departments: Arena::new(),
            employees: Arena::new(),
        }
    }
}

enum Asset {
    Company,
    Department,
    Employee,
}

struct Employee {
    name: String,
    department: DepartmentId,
}
type EmployeeId = generational_arena::Index;

struct Department {
    name: String,
}
type DepartmentId = generational_arena::Index;

struct Company {
    name: String,
    employees: LinkedHashSet<EmployeeId>,
    departments: LinkedHashSet<DepartmentId>,
}
type CompanyId = generational_arena::Index;

impl Department {
    fn new(name: &str) -> Department {
        Department {
            name: name.to_string(),
        }
    }
}

impl Employee {
    fn new(name: &str, department: DepartmentId) -> Employee {
        Employee {
            name: name.to_string(),
            department: department,
        }
    }
}

impl Company {
    fn new(name: &str) -> Company {
        Company {
            name: name.to_string(),
            employees: LinkedHashSet::new(),
            departments: LinkedHashSet::new(),
        }
    }
}

fn new_company(store: &mut DataStore) {
    if let Prompt::Value(name) = prompt("COMPANY NAME?\n> ") {
        let name = name.to_ascii_uppercase();
        if store
            .companies
            .iter()
            .find(|&(_, c)| c.name == name)
            .is_none()
        {
            store.companies.insert(Company::new(&name[..]));
        }
    }
}

fn select_employee(store: &DataStore, company: CompanyId) -> Option<EmployeeId> {
    let company = &store.companies[company];
    if let Prompt::Value(num) = prompt("EMPLOYEE #? ") {
        match num.parse::<usize>() {
            Ok(num) if 0 < num && num < company.employees.len() + 1 => {
                Some(*company.employees.iter().nth(num - 1).unwrap())
            }
            _ => None,
        }
    } else {
        None
    }
}

fn list_employees(store: &DataStore, company: CompanyId) {
    let company = &store.companies[company];
    if company.employees.is_empty() {
        println!("NO EMPLOYEES");
    } else {
        println!("EMPLOYEES:");
        for (i, id) in company.employees.iter().enumerate() {
            let employee = &store.employees[*id];
            let department = &store.departments[employee.department];
            println!("#{}: {} -> {}", i + 1, employee.name, department.name);
        }
    }
}

fn add_employee(store: &mut DataStore, company: CompanyId) {
    if store.companies[company].departments.is_empty() {
        println!("NO DEPARTMENTS");
        return;
    }
    if let Prompt::Value(name) = prompt("EMPLOYEE NAME?\n> ") {
        let name = name.to_ascii_uppercase(); // we should really put some more thought into this, it's likely not ok to uppercase employee names
                                              // also, names may not be unique
        match store.companies[company]
            .employees
            .iter()
            .position(|&e| store.employees[e].name == name)
        {
            None => {
                if let Some(department) = select_department(store, company) {
                    let id = store.employees.insert(Employee::new(&name[..], department));
                    store.companies[company].employees.insert(id);
                    println!(
                        "EMPLOYEE ADDED: #{} -> {}",
                        store.companies[company].employees.len(),
                        store.departments[department].name
                    );
                } else {
                    println!("NO SUCH DEPARTMENT");
                }
            }
            Some(num) => {
                println!("EMPLOYEE ALREADY EXIST: #{}", num + 1);
            }
        }
    }
}

fn move_employee(store: &mut DataStore, company: CompanyId) {
    if store.companies[company].employees.is_empty() {
        println!("NO EMPLOYEES");
        return;
    }
    if let Some(employee) = select_employee(store, company) {
        if let Some(department) = select_department(store, company) {
            store.employees[employee].department = department;
            println!(
                "EMPLOYEE MOVED: {} -> {}",
                store.employees[employee].name, store.departments[department].name
            );
        } else {
            println!("NO SUCH DEPARTMENT");
        }
    } else {
        println!("NO SUCH EMPLOYEE");
    }
}

fn remove_employee(store: &mut DataStore, company: CompanyId) {
    if store.companies[company].employees.is_empty() {
        println!("NO EMPLOYEES");
        return;
    }
    if let Some(employee) = select_employee(store, company) {
        let name = store.employees[employee].name.clone();
        let company = &mut store.companies[company];
        company.employees.remove(&employee);
        store.employees.remove(employee);
        println!("EMPLOYEE REMOVED: {}", name);
    } else {
        println!("NO SUCH EMPLOYEE");
    }
}

fn select_department(store: &DataStore, company: CompanyId) -> Option<DepartmentId> {
    let company = &store.companies[company];
    if let Prompt::Value(num) = prompt("DEPARTMENT #? ") {
        match num.parse::<usize>() {
            Ok(num) if 0 < num && num < company.departments.len() + 1 => {
                Some(*company.departments.iter().nth(num - 1).unwrap())
            }
            _ => None,
        }
    } else {
        None
    }
}

fn list_departments(store: &DataStore, company: CompanyId) {
    let company = &store.companies[company];
    if company.departments.is_empty() {
        println!("NO DEPARTMENTS");
    } else {
        println!("DEPARTMENTS:");
        for (i, &id) in company.departments.iter().enumerate() {
            let department = &store.departments[id];
            let count = company
                .employees
                .iter()
                .map(|&e| &store.employees[e])
                .filter(|e| e.department == id)
                .count();
            println!(
                "#{}: {} ({} EMPLOYEE{})",
                i + 1,
                department.name,
                count,
                if count == 1 { "" } else { "S" }
            );
        }
    }
}

fn add_department(store: &mut DataStore, company: CompanyId) {
    if let Prompt::Value(name) = prompt("DEPARTMENT NAME?\n> ") {
        let name = name.to_ascii_uppercase();
        match store.companies[company]
            .departments
            .iter()
            .position(|&d| store.departments[d].name == name)
        {
            None => {
                let id = store.departments.insert(Department::new(&name[..]));
                store.companies[company].departments.insert(id);
                println!(
                    "DEPARTMENT ADDED: #{}",
                    store.companies[company].departments.len()
                );
            }
            Some(num) => {
                println!("DEPARTMENT ALREADY EXISTS: #{}", num + 1);
            }
        }
    }
}

fn remove_department(store: &mut DataStore, company: CompanyId) {
    if store.companies[company].departments.is_empty() {
        println!("NO DEPARTMENTS");
        return;
    }
    if let Some(department) = select_department(store, company) {
        let name = store.departments[department].name.clone();
        let company = &mut store.companies[company];
        for employee in &company.employees {
            if store.employees[*employee].department == department {
                println!("DEPARTMENT IS NOT EMPTY");
                return;
            }
        }
        company.departments.remove(&department);
        store.departments.remove(department);
        println!("DEPARTMENT REMOVED: {}", name);
    } else {
        println!("NO SUCH DEPARTMENT");
    }
}

fn manage_company(store: &mut DataStore, company: CompanyId) {
    loop {
        let title = format!(" COMPANY {} ", &store.companies[company].name);
        println!("\n{:=^1$}", title, title.len() + 10);

        let mut choices = Vec::new();
        choices.push(Choice::List("EMPLOYEES", Asset::Employee));
        choices.push(Choice::Add("EMPLOYEE", Asset::Employee));
        choices.push(Choice::Move("EMPLOYEE", Asset::Employee));
        choices.push(Choice::Remove("EMPLOYEE", Asset::Employee));
        choices.push(Choice::List("DEPARTMENTS", Asset::Department));
        choices.push(Choice::Add("DEPARTMENT", Asset::Department));
        choices.push(Choice::Remove("DEPARTMENT", Asset::Department));
        choices.push(Choice::Cancel("BACK"));
        match choose(choices) {
            Choice::List(_, Asset::Employee) => {
                list_employees(store, company);
            }
            Choice::Add(_, Asset::Employee) => {
                add_employee(store, company);
            }
            Choice::Move(_, Asset::Employee) => {
                move_employee(store, company);
            }
            Choice::Remove(_, Asset::Employee) => {
                remove_employee(store, company);
            }
            Choice::List(_, Asset::Department) => {
                list_departments(store, company);
            }
            Choice::Add(_, Asset::Department) => {
                add_department(store, company);
            }
            Choice::Remove(_, Asset::Department) => {
                remove_department(store, company);
            }
            Choice::Cancel(_) => {
                break;
            }
            _ => {
                panic!("IMPOSSIBLE CHOICE!");
            }
        };
        getchar();
    }
}

fn manage_companies(store: &mut DataStore) {
    loop {
        println!("");
        println!("==========================");
        println!("HUMAN RESOURCES MANAGEMENT");
        println!("==========================");
        println!("");
        println!("WHICH COMPANY DO YOU WANT TO MANAGE?");

        let mut choices = Vec::new();
        for (i, comp) in store.companies.iter() {
            choices.push(Choice::Entry(&comp.name[..], Asset::Company, i));
        }
        choices.push(Choice::New("", Asset::Company));
        choices.push(Choice::Cancel("QUIT"));
        match choose(choices) {
            Choice::New(_, _) => {
                new_company(store);
            }
            Choice::Entry(_, _, i) => {
                manage_company(store, i);
            }
            Choice::Cancel(_) => {
                break;
            }
            _ => {
                panic!("IMPOSSIBLE CHOICE!");
            }
        };
    }
}

fn main() {
    let mut store = DataStore::new();
    manage_companies(&mut store);
}
