use std::error::Error;
pub mod config;
pub mod tools;
pub mod commands;
use config::Config;
use commands::command_list;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut command_list = command_list::CommandList::default();
    let keys: Vec<&String> = command_list.list.keys().collect();
    if !keys.contains(&&config.process) {
        Err("Command not found")?
    }
    let command = command_list.list.get_mut(&config.process).unwrap();
    command.set_from_args(&config.arguments)?;
    command.run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    
}