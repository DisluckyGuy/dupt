use std::{fs, process};

use crate::tools::{get_root_path, search_installed};

use super::Command;

pub struct Run {
    pub name: String,
}

impl Command for Run {
    fn help(&self) {
        println!("run");
        println!("run installed software");
    }

    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        search_installed(&self.name)?;

        let pkginfo =
            fs::read_to_string(format!("{}/.dupt/installed/{}", get_root_path(), self.name))?;

        let mut exec_path = String::new();
        for i in pkginfo.lines() {
            if !i.contains(":") {
                continue;
            }
            let line = i.split_once(":").unwrap();
            let key = &line.0.trim();
            let value = &line.1.trim();
            if key == &"exec" {
                exec_path = value.to_string();
            }
        }

        let exec_name: &str;
        let mut exec_dir = "";
        if exec_path.contains("/") {
            let exec_split = exec_path.rsplit_once("/").unwrap();
            exec_name = exec_split.1;
            exec_dir = exec_split.0;
        } else {
            exec_name = &exec_path;
        }

        let _run = process::Command::new("distrobox")
            .current_dir(format!("{}/.dupt/bin/{}/{}", get_root_path(), self.name, exec_dir))
            .arg("enter")
            .arg("dupt-fedora")
            .arg("--")
            .arg(format!("./{}", exec_name))
            .spawn()?
            .wait()?;
        Ok(())
    }

    fn set_from_args(&mut self, args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() == 0 {
            Err("not enought arguments")?
        }
        self.name = String::from(&args[0]);
        Ok(())
    }
}

impl Default for Run {
    fn default() -> Self {
        Self {
            name: String::from("help"),
        }
    }
}
