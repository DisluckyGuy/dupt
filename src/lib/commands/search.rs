use ansi_term::Color;

use crate::tools::paths;
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

    fn set_from_args(&mut self, args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() < 1 {
            Err("not enough arguments")?
        }

        self.name = String::from(&args[0]);

        Ok(())
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        let list_dir = fs::read_dir(format!("{}/sources/repositories", paths::get_project_dir()))?;
        let mut packages: Vec<Package> = Vec::new();
        for i in list_dir {
            let repo_file = fs::read_to_string(i.unwrap().path())?;
            let pkgs: &Vec<&str> = &repo_file.lines().collect();
            let mut in_package = false;
            let mut name = String::new();
            let mut version = String::new();
            let mut description_short = String::new();
            for j in pkgs {
                if j.trim().is_empty() {
                    continue;
                }

                if &j[0..4] != "    " && j.trim() != name && in_package {
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

                if j.trim().contains(&self.name) && &j[0..4] != "    " {
                    name = String::from(j.trim());
                    in_package = true;
                }

                if &j[0..4] == "    " && in_package {
                    let key = j.split_once(":").unwrap().0.trim();
                    let value = j.split_once(":").unwrap().1.trim();
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
        }

        if packages.len() == 0 {
            Err("no matching packages")?
        }

        for i in &packages {
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

impl Search {}

struct Package {
    pub name: String,
    pub version: String,
    pub description_short: String,
}
