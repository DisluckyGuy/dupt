use std::{env, fs};

use crate::tools::{
    self, containers,
    packages::{self, search_package},
    paths, terminal,
};

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
        containers::check_toolbox_env()?;

        containers::make_dupt_folder()?;

        for name in &self.names {
            if packages::search_installed(name).is_ok() {
                terminal::print_green("package already installed");
                return Ok(());
            }
        }

        terminal::print_blue("searching packages");

        for name in &self.names {
            search_package(name)?
        }

        println!("packages found");

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

        for name in &self.names {
            env::set_current_dir(paths::get_project_dir())?;

            terminal::print_blue("downloading package");

            packages::get_file(
                &format!("{}.tar.gz", name),
                &format!("{}.tar.gz", name),
                "dupt-repo-main",
                format!("{}/.dupt/archives", paths::get_root_path()),
            )?;
            println!("downloaded");

            let tar_file = fs::File::open(format!(
                "{}/.dupt/archives/{}.tar.gz",
                paths::get_root_path(),
                name
            ))?;

            let tar = flate2::read::GzDecoder::new(tar_file);
            let mut archive = tar::Archive::new(tar);
            println!("unpacking");
            archive.unpack(format!("{}/.dupt/archives", paths::get_root_path()))?;

            let pkginfo = fs::read_to_string(format!(
                "{}/.dupt/archives/{}/PKGINFO.conf",
                paths::get_root_path(),
                name
            ))?;
            let mut make_dependecies = Vec::new() as Vec<&str>;
            let mut dependecies = Vec::new() as Vec<&str>;
            let mut current_value = String::new();

            for i in pkginfo.lines() {
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

            let mut command = make_dependecies.join(" ");

            println!("{}", make_dependecies.len());

            terminal::print_blue("installing make dependencies");

            println!("sudo dnf install {} -y", command);

            containers::run_distrobox_command(&format!("sudo dnf install {} -y", command), true)?;

            env::set_current_dir(format!(
                "{}/.dupt/archives/{}/control",
                paths::get_root_path(),
                name
            ))?;

            println!();
            println!("running preinstall configurations");

            containers::run_distrobox_command(
                &format!("sh preinst.sh {}", paths::get_root_path()),
                true,
            )?;

            println!();
            terminal::print_blue("building..");

            containers::run_distrobox_command(
                &format!("sh build.sh {}", paths::get_root_path()),
                true,
            )?;

            terminal::print_blue("removing make dependencies");

            containers::run_distrobox_command(&format!("sudo dnf remove {} -y", command), true)?;

            command.clear();

            for i in &dependecies {
                command += i;
                command += " ";
            }

            terminal::print_blue("installing dependencies");

            tools::containers::run_distrobox_command(
                &format!("sudo dnf install {} -y", command),
                true,
            )?;

            println!();
            println!("running post configurations");

            containers::run_distrobox_command(
                &format!("sh preinst.sh {}", paths::get_root_path()),
                true,
            )?;

            containers::run_distrobox_command(
                &format!(
                    "cp {0}/.dupt/archives/{1}/PKGINFO.conf {0}/.dupt/installed/{1}",
                    paths::get_root_path(),
                    name
                ),
                false,
            )?;
        }

        println!("cleaning archives");
        for name in &self.names {
            packages::clear_archives(&name)?;
        }

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
