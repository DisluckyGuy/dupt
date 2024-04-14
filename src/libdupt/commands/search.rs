use ansi_term::Color;

use crate::{package_data::PackageData, tools::paths};
use std::{error::Error, fs, process::exit};
pub struct Search {
    pub name: String,
}

impl super::Command for Search {

}

impl Default for Search {
    fn default() -> Self {
        Search {
            name: String::from("help"),
        }
    }
}

impl Search {
    fn help(&self) {
    println!("search:");
    println!();
    println!("a command used to search packages");
}

pub fn from_args(args: &Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
    let mut command = Search::default();
    if args.len() < 1 {
        command.help();
        Err("not enough arguments")?
    }

    command.name = String::from(&args[0]);

    if command.name == "help".to_string() {
        command.help();
        exit(0);
    }

    Ok(command)
}

pub fn run(&self) -> Result<Vec<PackageData>, Box<dyn Error>> {
    let list_dir = fs::read_dir(format!("{}/sources/repositories", paths::get_project_dir()))?;
    let mut packages: Vec<PackageData> = Vec::new();
    for i in list_dir {
        let file = fs::read_to_string(i.unwrap().path())?;
        let pkgs: Vec<PackageData> = serde_json::from_str(&file)?;
        for j in pkgs {
            if j.package_name.contains(&self.name) {
                packages.push(j);
            }
        }
    }

    if packages.len() == 0 {
        Err("no matching packages")?
    }

    for i in &packages {
        let termsize = usize::from(termsize::Size::from(termsize::get().unwrap()).cols);
        let remain_len = termsize - (&i.package_name.len());
        println!("{}{:>remain_len$}", i.package_name, i.version);
        println!(
            "{}",
            Color::RGB(100, 100, 100).paint(format!("{}", i.summary))
        );
    }

    Ok(packages)
}}