use std::collections::HashMap;
use std::io;
#[derive(Debug)]
enum Command {
    Add {
        names: Vec<String>,
        department: String,
    },
    Get(String),
    GetAll,
    Remove {
        names: Vec<String>,
        department: String,
    },
    Quit,
}
impl Command {
    fn parse_command(command_parts: &Vec<&str>) -> Option<Command> {
        let action = command_parts[0];
        match action {
            "add" => Some(Command::Add {
                names: vec![command_parts[1].to_owned()],
                department: command_parts[command_parts.len() - 1].to_owned(),
            }),
            "get" => None,
            _ => {
                println!("Unknown command");
                None
            }
        }
    }
}

fn main() {
    println!("Welcome to the Department Lookup!");
    println!("What would you like to do? [add, get, or remove] employee(s)");
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let raw_input = get_raw_input();
        let command_parts: Vec<&str> = raw_input.split_whitespace().collect();
        let command = Command::parse_command(&command_parts);
        // let command = Command::parse_command(&command_parts);
        // let action = Command::get_action(command_parts[0]);
        // if action.is_none() {
        //     continue;
        // }
        // if let Some(cmd) = action {
        //     match cmd {
        //         Command::Add => add(&mut departments),
        //         Command::Get => get(&departments),
        //         Command::Remove => remove(&mut departments),
        //     }
        // }
    }
}

fn get_raw_input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to get the input");
    buffer.trim().to_owned()
}

fn add(departments: &mut HashMap<String, Vec<String>>) {}
fn get(departments: &HashMap<String, Vec<String>>) {}
fn remove(departments: &mut HashMap<String, Vec<String>>) {}
