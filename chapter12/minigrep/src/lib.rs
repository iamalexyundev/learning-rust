use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("Usage `minigrep [query] [filepath]`");
        }
        Ok(Config {
            query: &args[1],
            file_path: &args[2],
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(config.query, &contents) {
        println!("{line}")
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line.trim());
        }
    }
    result
}

//inefficient way
// struct Config {
//     query: String,
//     file_path: String,
// }
// fn parse_config(args: &[String]) -> Config {
//     Config {
//         query: args[1].clone(),
//         file_path: args[2].clone(),
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}
