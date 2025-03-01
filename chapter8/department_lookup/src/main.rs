use std::collections::HashMap;
use std::io;
#[derive(Debug)]
enum Command {
    Add { name: String, department: String },
    Get(String),
    GetAll,
    Quit,
}
impl Command {
    fn parse_command(command_parts: Vec<&str>) -> Option<Command> {
        if command_parts.is_empty() {
            println!("Please input a command `add` or `get`");
            return None;
        }
        let action = command_parts[0].to_lowercase();
        match action.as_str() {
            "add" => parse_add(&command_parts),
            "get" => parse_get(&command_parts),
            "q" | "quit" => Some(Command::Quit),
            _ => {
                println!("Unknown command: Use `add` or `get`");
                None
            }
        }
    }
}

fn main() {
    println!("Welcome to the Department Lookup!");
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("What would you like to do? [add or get] employee(s)");
        let raw_input = get_raw_input();
        let command_parts: Vec<&str> = raw_input.split_whitespace().collect();
        let command = Command::parse_command(command_parts);
        match command {
            None => {
                continue;
            }
            Some(command) => {
                match command {
                    Command::Add { name, department } => {
                        departments
                            .entry(department) //should i clone here?
                            .or_default()
                            .push(name); //should i clone here?
                        // println!("Added {name} to {department}")
                    }
                    Command::Get(department) => {
                        let names = departments.get(&department);
                        match names {
                            Some(names) => {
                                println!("List of all employees from {department}:");
                                names.iter().for_each(|name| println!("{name}"));
                            }
                            None => {
                                println!("The `{department}` department does not exist");
                                continue;
                            }
                        }
                    }
                    Command::GetAll => {
                        let mut all_employees = vec![];
                        departments
                            .iter()
                            .for_each(|department| all_employees.extend(department.1));
                        println!("List of all employees from all departments:");
                        all_employees.sort_by_key(|s| s.to_lowercase());
                        all_employees.iter().for_each(|name| println!("{name}"));
                    }
                    Command::Quit => std::process::exit(1),
                }
            }
        }
    }
}

fn get_raw_input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to get the input");
    buffer.trim().to_owned()
}

fn parse_add(command_parts: &[&str]) -> Option<Command> {
    if command_parts.len() != 4 || command_parts[2] != "to" {
        println!("Usage: `add name to department`");
        None
    } else {
        let name = command_parts[1].to_owned();
        let department = command_parts[3].to_owned();
        Some(Command::Add { name, department })
    }
}

fn parse_get(command_parts: &[&str]) -> Option<Command> {
    match command_parts.len() {
        2 => Some(Command::GetAll),
        4 => {
            if command_parts.contains(&"from") {
                Some(Command::Get(command_parts[3].to_owned()))
            } else {
                println!("Usage: `get all from department`");
                None
            }
        }
        _ => {
            println!("Usage: `get all from department` or `get all`");
            None
        }
    }
}
