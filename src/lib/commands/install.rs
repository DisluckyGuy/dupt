use std::{env, fs};

use crate::tools::{self, containers, packages, paths, terminal};

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

        terminal::print_blue("searching package");

        packages::search_package(&self.names[0])?;

        containers::check_toolbox_env()?;

        containers::make_dupt_folder()?;

        println!("package found");

        if self.confirm {
            println!();
            println!("packages to install:");
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

        terminal::print_blue("downloading package");

        packages::get_file(
            &format!("{}.tar.gz", &self.names[0]),
            "dupt-repo-main",
            format!("{}/.dupt/archives", paths::get_root_path()),
            "main",
        )?;

        println!("downloaded");

        let tar_file = fs::File::open(format!(
            "{}/.dupt/archives/{}.tar.gz",
            paths::get_root_path(),
            self.names[0]
        ))?;

        let tar = flate2::read::GzDecoder::new(tar_file);
        let mut archive = tar::Archive::new(tar);
        println!("unpacking");
        archive.unpack(format!("{}/.dupt/archives", paths::get_root_path()))?;

        let pkginfo = fs::read_to_string(format!("{}/.dupt/archives/{}/PKGINFO.conf", paths::get_root_path(), self.names[0]))?;

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

        terminal::print_blue("installing make dependencies");

        println!("sudo dnf install {} -y", command);

        containers::run_distrobox_command(&format!("sudo dnf install {} -y", command), true)?;

        env::set_current_dir(format!(
            "{}/.dupt/archives/{}/control",
            paths::get_root_path(),
            self.names[0]
        ))?;

        println!();
        println!("running preinstall configurations");

        containers::run_distrobox_command(&format!("sh preinst.sh {}", paths::get_root_path()), true)?;

        println!();
        terminal::print_blue("building..");

        containers::run_distrobox_command(&format!("sh build.sh {}", paths::get_root_path()), true)?;

        terminal::print_blue("removing make dependencies");

        containers::run_distrobox_command(&format!("sudo dnf remove {} -y", command), true)?;

        command.clear();
        
        for i in &dependecies {
            command += i;
            command += " ";
        }

        terminal::print_blue("installing dependencies");

        tools::containers::run_distrobox_command(&format!("sudo dnf install {} -y", command), true)?;

        println!();
        println!("running post configurations");

        containers::run_distrobox_command(&format!("sh preinst.sh {}", paths::get_root_path()), true)?;

        containers::run_distrobox_command(&format!("cp {0}/.dupt/archives/{1}/PKGINFO.conf {0}/.dupt/installed/{1}", paths::get_root_path(), self.names[0]), false)?;

        println!("cleaning archives");

        packages::clear_archives(&self.names[0])?;

        println!();
        terminal::print_green("finished successfully");
        Ok(())
    }

    fn set_from_args(&mut self, args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() == 0 {
            self.help();
            return Err("Not enough arguments")?;
        }

        if args.last().unwrap() == "-y" {
            self.confirm = false;
        }

        if args.len() == 1 && self.confirm == false {
            self.help();
            return Err("Not enough arguments")?;
        }

        if !self.confirm {
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
            confirm: true,
        }
    }
}
