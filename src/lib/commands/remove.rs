use std::fs;

use crate::tools::{containers, packages, paths, terminal};

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

        containers::make_dupt_folder()?;

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
