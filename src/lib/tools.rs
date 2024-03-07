use std::{env, error::Error, io::{self, Write}};

use crate::package::Package;


pub fn get_project_dir() -> String {
    let current_exec = env::current_exe().unwrap();
    let project_dir: Vec<&str> = current_exec.to_str().unwrap().rsplitn(4, "/").collect();
    String::from(project_dir[project_dir.len() - 1])
}

pub fn confirm() -> Result<bool, Box<dyn Error>> {
    print!("Do you want to continue? [y/n]: ");
    io::stdout().flush()?;
    let mut confirm = String::new();
    std::io::stdin().read_line(&mut confirm)?;
    confirm = String::from(confirm.trim());

    if confirm == "n" {
        println!("aborting...");
        return Ok(false);
    } else if confirm != "y" {
        return Err("invalid value. aborting...")?;
        
    }
    
    Ok(true)
}

pub fn search_package<'a>(contents: &'a str, package: &str) -> Result<Package, &'static str> {
    let mut in_install = false;
    let mut in_run = false;
    let mut name = "";
    let mut zip = "";
    let mut description = String::from("");
    let mut install: Vec<String> = Vec::new();
    let mut runnable = false;
    let mut run: Vec<String> = Vec::new();

    for i in contents.lines() {
        if !i.contains(package) {
            continue;
        }
        for j in i.char_indices() {
            if j.1 == ':' {
                name = &i[0..j.0];
                zip = &i[j.0 + 2..i.len()];
            }
        }
        if name == package.to_lowercase() {
            let pkg_file = std::fs::read_to_string(format!(
                "{}/packages/zips/{}/{}.config",
                get_project_dir(),
                zip,
                name
            ))
            .unwrap();
            for i in pkg_file.lines() {
                if i.trim() == "]" && in_install == true {
                    in_install = false;
                    continue;
                } else if i.trim() == "]" && in_run == true{
                    in_run = false;
                    continue;
                }
                if in_install {
                    install.push(String::from(i.trim_start().trim_end()));
                    continue;
                } else if in_run {
                    run.push(String::from(i.trim_start().trim_end()));
                    continue;
                }
                
                let separator = i.find(":").unwrap();
                
                if &i[0..separator] == "name" {
                    name = &i[separator + 2..i.len()];
                } else if &i[0..separator] == "zip" {
                    zip = &i[separator + 2..i.len()];
                } else if &i[0..separator] == "description" {
                    description = i[separator + 2..i.len()].to_string();
                } else if &i[0..separator] == "install" {
                    in_install = true;
                    continue;
                } else if &i[0..separator] == "runnable" {
                    if i[separator + 2..i.len()].trim() == "true" {
                        runnable = true;
                    }
                } else if &i[0..separator] == "run" {
                    in_run = true;
                    continue;
                }
            }
            return Ok(Package {
                name: String::from(name),
                zip: String::from(zip),
                description: description,
                install: install,
                runnable: runnable,
                run: run
            });
        }
    }

    Err("package not found")
}
