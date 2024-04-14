use std::{fs, process::exit};

use crate::tools::{containers, packages, paths, terminal};

use super::Command;

pub struct Remove {
    names: Vec<String>,
    confirm: bool,
}

impl Command for Remove {
}

impl Default for Remove {
    fn default() -> Self {
        Self {
            names: vec!["help".to_string()],
            confirm: true,
        }
    }
}

impl Remove {
    fn help(&self) {
        println!("remove installed packages");
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {

        containers::make_dupt_folder()?;

        if self.names.contains(&"help".to_string())  {
            self.help();
        }

        for name in &self.names {
            packages::search_installed(name)?;
        }

        if self.confirm {
            println!();
            println!("packages to remove");
            println!();

            for i in &self.names {
                println!("{}", i);
            }
            println!();

            let cont = terminal::confirm("Do you want to continue? [y/n]:")?;
            println!();
            if !cont {
                println!();
                println!("aborting...");
                return Ok(());
            }
        }

        for name in &self.names {

            fs::remove_dir_all(format!("{}/.dupt/bin/{}", paths::get_root_path(), name))?;

            let unused_dep = packages::get_unused_dependencies(&name)?;
            let unused_str = &unused_dep.join(" ");

            containers::run_distrobox_command(&format!("sudo dnf remove {} -y", unused_str), true)?;
            fs::remove_file(format!("{}/.dupt/installed/{}", paths::get_root_path(), name))?;
        }
        
        

        terminal::print_green("removed succesfully!");

        Ok(())
    }

    pub fn from_args(args: &Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut command = Remove::default();

        
        if args.len() == 0 {
            command.help();
            return Err("not enough arguments")?;
        }

        if args[args.len() - 1] != "-y" {
            command.names = args[0..args.len()].to_vec();
            return Ok(command);
        }

        command.confirm = false;
        command.names = args[0..args.len()].to_vec();
        if command.names.contains(&"help".to_string()) {
            command.help();
            exit(0);
        }
        Ok(command)
    }
}