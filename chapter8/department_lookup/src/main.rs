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
        let action = command_parts[0].to_lowercase();
        match action.as_str() {
            "add" => parse_add(&command_parts),
            "get" => parse_get(&command_parts),
            "q" | "quit" => Some(Command::Quit),
            _ => {
                println!("Unknown command");
                None
            }
        }
    }
}

fn main() {
    println!("Welcome to the Department Lookup!");
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("What would you like to do? [add, get, or remove] employee(s)");
        let raw_input = get_raw_input();
        let command_parts: Vec<&str> = raw_input.split_whitespace().collect();
        let command = Command::parse_command(command_parts);
        if command.is_none() {
            continue;
        }
        match command.unwrap() {
            Command::Add { name, department } => {
                departments.entry(department).or_default().push(name);
            }
            Command::Quit => std::process::exit(1),
            _ => todo!(),
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

fn parse_add(command_parts: &Vec<&str>) -> Option<Command> {
    if command_parts.len() < 4 || !command_parts.contains(&"to") || command_parts.len() > 4 {
        println!("Wrong format! Usage: add name to department");
        None
    } else {
        let name = command_parts[1].to_owned();
        let department = command_parts[3].to_owned();
        Some(Command::Add { name, department })
    }
}

fn parse_get(command_parts: &Vec<&str>) -> Option<Command> {
    Some(Command::Get("GWET".to_owned()))
}
