use std::process::exit;




use crate::tools::{containers, packages::{self, search_installed}, terminal};

use super::Command;
pub struct PkgInfo {
    name: String,
}

impl Command for PkgInfo {
    
}

impl Default for PkgInfo {
    fn default() -> Self {
        Self {
            name: String::from("help"),
        }
    }
}

impl PkgInfo {
    pub fn help(&self) {
        todo!()
    }

    pub fn run(&self) -> Result<String, Box<dyn std::error::Error>> {

        println!("running");
        containers::check_toolbox_env()?;
        containers::make_dupt_folder()?;

        println!("finished checking");

        if let Ok(_packages) = search_installed(&self.name) {
            println!("searching");
            let package = search_installed(&self.name)?;
            println!("search successful");
            terminal::print_green("retrieving info successful");
            return Ok(format!("{:#?}", package).replace("\"", "").replace("    ", ""));
        } else {
            let package = packages::search_package(&self.name)?;
            terminal::print_green("retrieving info successful");
            Ok(format!("{:#?}", package).replace("\"", "").replace("    ", ""))
        }
        
    }

    pub fn from_args(args: &Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let command = PkgInfo::default();
        if args.len() == 0 {
            return Err("not enough arguments")?;
        }

        if command.name == "help".to_string() {
            command.help();
            exit(0);
        }

        Ok(PkgInfo{name: args[0].to_string()})
    }
}