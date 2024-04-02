use std::{env, fs};

use crate::tools::{self, clear_archives, get_root_path, print_green, run_distrobox_command};

use super::Command;
pub struct Install {
    pub names: Vec<String>,
    pub confirm: bool,
}

impl Command for Install {
    fn help(&self) {
        println!("install");
        println!();
        println!("install software from different repositories");
    }

    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {

        tools::print_blue("searching package");

        tools::search_package(&self.names[0])?;

        tools::check_toolbox_env()?;

        tools::make_dupt_folder()?;

        println!("package found");

        tools::print_blue("downloading package");

        tools::get_file(
            &format!("{}.tar.gz", &self.names[0]),
            "dupt-repo-main",
            format!("{}/.dupt/archives", tools::get_root_path()),
            "main",
        )?;

        println!("downloaded");

        let tar_file = fs::File::open(format!(
            "{}/.dupt/archives/{}.tar.gz",
            tools::get_root_path(),
            self.names[0]
        ))?;

        let tar = flate2::read::GzDecoder::new(tar_file);
        let mut archive = tar::Archive::new(tar);
        println!("unpacking");
        archive.unpack(format!("{}/.dupt/archives", tools::get_root_path()))?;

        let pkginfo = fs::read_to_string(format!("{}/.dupt/archives/{}/PKGINFO.conf", get_root_path(), self.names[0]))?;

        let mut make_dependecies = Vec::new() as Vec<&str>;
        let mut dependecies = Vec::new() as Vec<&str>;
        let mut current_value = String::new();

        for i in pkginfo.lines() {
            println!("current value: {}", current_value);
            if i.trim().is_empty() {
                continue;
            }
            if i.trim() == "]" {
                continue;
            }
            if &i[0..4] == "    " {
                if current_value == "make_dependencies" {
                    make_dependecies.push(i.trim())
                } else if current_value == "dependencies" {
                    dependecies.push(i.trim());
                }
                continue;
            }
            println!("{}", i);
            let line = i.split_once(":").unwrap();
            let key = &line.0;
            //let value = &line.1;
            current_value = key.to_string();
        }

        let mut command = String::new();

        println!("{}", make_dependecies.len());
        for i in &make_dependecies {
            command += i;
            command += " ";
        }

        tools::print_blue("installing make dependencies");

        println!("sudo dnf install {} -y", command);

        tools::run_distrobox_command(&format!("sudo dnf install {} -y", command), true)?;

        env::set_current_dir(format!(
            "{}/.dupt/archives/{}/control",
            tools::get_root_path(),
            self.names[0]
        ))?;

        println!();
        println!("running preinstall configurations");

        run_distrobox_command(&format!("sh preinst.sh {}", get_root_path()), true)?;

        println!();
        tools::print_blue("building..");

        run_distrobox_command(&format!("sh build.sh {}", get_root_path()), true)?;

        tools::print_blue("removing make dependencies");

        run_distrobox_command(&format!("sudo dnf remove {} -y", command), true)?;

        command.clear();
        
        for i in &dependecies {
            command += i;
            command += " ";
        }

        tools::print_blue("installing dependencies");

        tools::run_distrobox_command(&format!("sudo dnf install {} -y", command), true)?;

        println!();
        println!("running post configurations");

        run_distrobox_command(&format!("sh preinst.sh {}", get_root_path()), true)?;

        run_distrobox_command(&format!("cp {0}/.dupt/archives/{1}/PKGINFO.conf {0}/.dupt/installed/{1}", get_root_path(), self.names[0]), false)?;

        println!("cleaning archives");

        clear_archives(&self.names[0])?;

        println!();
        print_green("finished successfully");
        Ok(())
    }

    fn set_from_args(&mut self, args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() == 0 {
            self.help();
            return Err("Not enough arguments")?;
        }

        if args.last().unwrap() == "-y" {
            self.confirm = true;
        } else {
            self.confirm = false;
        }

        if args.len() == 1 && self.confirm == true {
            self.help();
            return Err("Not enough arguments")?;
        }

        if self.confirm == true {
            self.names = args[0..args.len() - 1].to_vec();
        } else {
            self.names = args.to_vec();
        }
        

        Ok(())
    }
}

impl Default for Install {
    fn default() -> Self {
        Self {
            names: vec![String::from("help")],
            confirm: false,
        }
    }
}
