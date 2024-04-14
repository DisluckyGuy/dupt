use std::process::{self, exit};

use crate::tools::{containers, packages::search_installed, paths};

use super::Command;

pub struct Run {
    pub name: String,
}

impl Command for Run {

}

impl Default for Run {
    fn default() -> Self {
        Self {
            name: String::from("help"),
        }
    }
}

impl Run {
    fn help(&self) {
        println!("run");
        println!("run installed software");
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {

        containers::check_toolbox_env()?;
        containers::make_dupt_folder()?;

        let exec_path = search_installed(&self.name)?.exec_dir;

        let exec = exec_path.rsplit_once("/").unwrap(); 

        let _run = process::Command::new("distrobox")
            .current_dir(format!("{}/.dupt/bin/{}/{}", paths::get_root_path(), self.name, exec.0))
            .arg("enter")
            .arg("dupt-fedora")
            .arg("--")
            .arg(format!("./{}", exec.1))
            .spawn()?
            .wait()?;
        Ok(())
    }

    pub fn from_args(args: &Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut command = Self::default();
        if args.len() == 0 {
            Err("not enought arguments")?
        }
        command.name = String::from(&args[0]);
        if command.name == "help".to_string() {
            command.help();
            exit(0);
        }
        Ok(command)
    }

}
