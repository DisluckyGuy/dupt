use std::fs;

use crate::tools::{containers, packages::{self, get_file}, paths::get_project_dir};

use super::Command;

pub struct Update {
    confirm: bool
}
 impl Command for Update {
    fn help(&self) {
        
    }
 
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        containers::check_toolbox_env()?;
        containers::make_dupt_folder()?;

        let repositories = packages::get_repos();
        let repo_dir = fs::read_dir(format!("{}/sources/repositories", get_project_dir()))?;
        for i in repo_dir {
            fs::remove_file(i.unwrap().path())?;
        }
        for i in repositories.keys() {
            get_file(&"list.conf".to_string(), &format!("{}.conf", i), i.as_str(), format!("{}/sources/repositories", get_project_dir()))?;
        }
        Ok(())
    }
 
    fn  set_from_args(&mut self, args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.confirm = true;
        if args.is_empty() {
            return Ok(());
        }
        if args[args.len()] == "-y" {
            self.confirm = true;
        }
        if args.contains(&"help".to_string()) {
            self.help();
        }
        Ok(())
    }
 }

 impl Default for Update {
    fn default() -> Self {
        Self { confirm: true }
    }
 }