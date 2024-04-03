use std::fs;

use crate::tools::{self, get_root_path, search_installed};

use super::Command;

pub struct Remove {
    names: Vec<String>,
    confirm: bool,
}

impl Command for Remove {
    fn help(&self) {
        todo!()
    }

    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        search_installed(&self.names[0])?;
        if self.confirm {
            println!();
            println!("packages to remove");
            println!();

            for i in &self.names {
                println!("{}", i);
            }
            println!();

            let cont = tools::confirm("Do you want to continue? [y/n]:")?;
            println!();
            if !cont {
                println!();
                println!("aborting...");
                return Ok(());
            }
        }
        fs::remove_dir_all(format!("{}/.dupt/bin/{}", get_root_path(), self.names[0]))?;
        fs::remove_file(format!("{}/.dupt/installed/{}", get_root_path(), self.names[0]))?;
        tools::print_green("removed succesfully!");

        Ok(())
    }

    fn set_from_args(&mut self, args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() == 0 {
            return Err("not enough arguments")?;
        }

        if args[args.len() - 1] != "-y" {
            self.names = args[0..args.len()].to_vec();
            return Ok(());
        }

        self.confirm = false;
        self.names = args[0..args.len()].to_vec();
        
        Ok(())
    }
}

impl Default for Remove {
    fn default() -> Self {
        Self {
            names: vec!["help".to_string()],
            confirm: true,
        }
    }
}
