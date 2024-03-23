use std::error::Error;
pub mod config;
pub mod tools;
pub mod commands;
use commands::Command;
use config::Config;
use std::collections;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut command_list: collections::HashMap<String, Box<&mut dyn Command>> = collections::HashMap::new();
    let mut search = commands::search::Search::default();
    command_list.insert(String::from("search"), Box::new(&mut search));
    let keys: Vec<&String> = command_list.keys().collect();
    if !keys.contains(&&config.process) {
        Err("Command not found")?
    }
    let command = &command_list[&config.process];
    command.run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    
}