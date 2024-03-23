use ansi_term::Color;

use crate::{config::Config, tools};
use std::{error::Error, fs};
pub struct Search {
    pub name: String,
}

impl super::Command for Search {
    fn help(&self) {
        println!("search:");
        println!();
        println!("a command used to search packages");
    }

    fn build<'a> (&'a self, config: &Config) -> Result<Box<dyn super::Command >, Box<dyn std::error::Error >> {
        if config.arguments.len() < 1 {
            Err("not enough arguments")?
        }

        let name = String::from(&config.arguments[0]);

        Ok(Box::new(Search { name }))
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        let pkgs_file =
            fs::read_to_string(format!("{}/sources/list.config", tools::get_project_dir()))?;
        let pkgs: &Vec<&str> = &pkgs_file.lines().collect();
        let mut in_package = false;
        let mut packages: Vec<Package> = Vec::new();
        let mut name = String::new();
        let mut version = String::new();
        let mut description_short = String::new();
        if self.name == "help" {
            self.help();
            return Ok(());
        }
        for i in pkgs {
            if i.trim().is_empty() {
                continue;
            }

            if &i[0..4] != "    " && i.trim() != name && in_package {
                in_package = false;
                packages.push(Package {
                    name: name.clone(),
                    version,
                    description_short: description_short.clone(),
                });
                name.clear();
                version = String::new();
                description_short.clear();
            }

            if i.trim().contains(&self.name) {
                name = String::from(i.trim());
                in_package = true;
            }

            if &i[0..4] == "    " && in_package {
                let key = i.split_once(":").unwrap().0.trim();
                let value = i.split_once(":").unwrap().1.trim();
                if key == "version" {
                    version = String::from(value);
                } else if key == "description_short" {
                    description_short = String::from(value);
                }
            }
        }
        if in_package {
            packages.push(Package {
                name: name.clone(),
                version: version.clone(),
                description_short: description_short.clone(),
            });
        }

        if packages.len() == 0 {
            Err("no matching packages")?
        }

        for i in packages {
            let termsize = usize::from(termsize::Size::from(termsize::get().unwrap()).cols);
            let remain_len = termsize - (&i.name.len());
            println!("{}{:>remain_len$}", i.name, i.version);
            println!(
                "{}",
                Color::RGB(100, 100, 100).paint(format!("{}", i.description_short))
            );
        }
        Ok(())
    }
}

impl Default for Search {
    fn default() -> Self {
        Search {
            name: String::from("help"),
        }
    }
}

impl Search {
}

struct Package {
    pub name: String,
    pub version: String,
    pub description_short: String,
}
