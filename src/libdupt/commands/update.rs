use std::{fs, process::exit};

use crate::tools::{containers, packages::{self, get_file}, paths::get_project_dir};

use super::Command;

pub struct Update {
    confirm: bool
}
 impl Command for Update {
    
 }

 impl Default for Update {
    fn default() -> Self {
        Self { confirm: true }
    }
 }

impl Update {
    fn help(&self) {
        println!("update package list");
    }
 
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        containers::check_toolbox_env()?;
        containers::make_dupt_folder()?;

        let repositories = packages::get_repos();
        let repo_dir = fs::read_dir(format!("{}/sources/repositories", get_project_dir()))?;
        for i in repo_dir {
            fs::remove_file(i.unwrap().path())?;
        }
        for i in repositories.keys() {
            get_file(&"list.conf".to_string(), &format!("{}.json", i), i.as_str(), format!("{}/sources/repositories", get_project_dir()))?;
        }
        Ok(())
    }
 
    pub fn from_args(args: &Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut command = Self::default();
        command.confirm = true;
        if args.is_empty() {
            return Ok(command);
        }
        if args.last().unwrap() == "-y" {
            command.confirm = true;
        }
        if args.contains(&"help".to_string()) {
            command.help();
            exit(0);
        }
        Ok(command)
    }
}