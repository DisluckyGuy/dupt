use std::error::Error;
use std::io::{self, Write};
use std::{fs, process};

pub struct Config {
    process: String,
    arguments: Vec<String>
}

pub struct Package {
    name: String,
    zip: String,
    commands: Vec<String>
}

impl Package {
    pub fn install(self) -> Result<(), Box<dyn Error>> {

        println!("package found!");
        println!("packages to install: ");
        println!();
        println!("{}",self.name);
        println!();
    
        print!("Do you want to continue? [y/n]: ");
        io::stdout().flush()?;
        let mut confirm = String::new();
        std::io::stdin().read_line(&mut confirm)?;
        confirm = String::from(confirm.trim());
    
        if confirm == "n" {
            println!("aborting...");
            return Ok(());
        } else if confirm != "y" {
            return Err("invalid value. aborting...")?;
        }
        
        for i in self.commands {
            let args: Vec<&str> = i.split_whitespace().collect();
            println!("running command: {}", args[0]);
            let _temp = process::Command::new(args[0])
            .args(args.split_at(1).1)
            .output()?;
            //println!("{}", String::from_utf8(_temp.stdout)?);
        }
    
        println!("installed package: {}", self.name);
    
        let mut fb = fs::OpenOptions::new()
            .append(true)
            .write(true)
            .open("packages/installed.config")?;
        let line = format!("{}: {}", self.name, self.zip);
        writeln!(fb, "{line}")?;
        Ok(())
    
    }

    pub fn remove (&self) -> Result<(), Box<dyn Error>> {

        println!("package found!");
        println!("packages to remove: ");
        println!();
        println!("{}",self.name);
        println!();
    
        print!("Do you want to continue? [y/n]: ");
        io::stdout().flush()?;
        let mut confirm = String::new();
        std::io::stdin().read_line(&mut confirm)?;
        confirm = String::from(confirm.trim());
    
        if confirm == "n" {
            println!("aborting...");
            return Ok(());
        } else if confirm != "y" {
            return Err("invalid value. aborting...")?;
        }
    
        let _rmdir = process::Command::new("rm")
        .arg(format!("packages/programs/{}/{}", self.zip, self.zip))
        .arg("--r")
        .arg("--f")
        .output()?;
    
        println!("removing..");
        println!();
        println!("removed package: {}", self.name);
    
        let install_list = std::fs::read_to_string("packages/installed.config")?;
        std::fs::write("packages/installed.config", "")?;
        let mut fb = fs::OpenOptions::new()
        .append(true)
        .write(true)
        .open("packages/installed.config")?;
        for i in install_list.lines() {
            if i.trim_end() == format!("{}: {}", self.name, self.zip) {
                continue;
            } else {
                writeln!(fb, "{i}")?;
            }
        }
    
        Ok(())
    }

}


impl Config{
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
    if args.len() == 1 {
        println!("ugs package manager for niche software!");
        std::process::exit(0);
    }
    let process = args[1].clone();
    if process == "install" {
        if args.len() == 2 {
            return Err("two few arguments for install process");
        }
    }
    let arguments = if args.len() > 2 {args.split_at(2).1.to_vec()} else {Vec::new()};
    Ok(Config{process, arguments})
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let package_list = std::fs::read_to_string("packages/list.config")?;
    let installed_list = std::fs::read_to_string("packages/installed.config")?;
    if config.process == "install" {
        let package = search_package(&package_list, &config.arguments[0])?;
        package.install()?;
        return Ok(());
    } else if config.process == "remove" {
        let package = search_package(&installed_list, &config.arguments[0])?;
        package.remove()?;
        return Ok(());
    } else if config.process == "list" {
        for i in package_list.lines() {
            let index = i.find(":").unwrap();
            println!("{}",&i[0..index]);
        }
        return Ok(());
    }
    Err("invalid command")?
}

pub fn search_package<'a> (contents: &'a str, package: &str) -> Result<Package, &'static str> {
    let mut in_commands = false;
    let mut name = "";
    let mut zip = "";
    let mut commands :Vec<String> = Vec::new();

    for i in contents.lines() {
        if i.contains(package) {
            for j in i.char_indices() {
                if j.1 == ':' {
                 name = &i[0..j.0];
                 zip = &i[j.0 + 2..i.len()];
                }
            }
            if name == package.to_lowercase() {
                let pkg_file = std::fs::read_to_string(format!("packages/programs/{}/{}.config", zip, name)).unwrap();
                for i in pkg_file.lines() {
                    if i.trim() == "]" && in_commands == true {
                        in_commands = false;
                    }
                    if in_commands {
                        commands.push(String::from(i
                            .trim_start()
                            .trim_end()
                            )
                        ); 
                    }
                    for j in i.char_indices() {
                        
                        if j.1 == ':' {
                            if &i[0..j.0] == "name" {
                                name = &i[j.0 + 2..i.len()];
                            } else if &i[0..j.0] == "zip"{
                                zip = &i[j.0 + 2..i.len()];
                            } else if &i[0..j.0] == "commands" {
                                in_commands = true;
                                continue;
                            }
                        }
                    }
                }
                println!("{}, {}, {:?}",name, zip, commands);
                return Ok(Package{name: String::from(name), zip: String::from(zip), commands: commands});
            }
        }
    }

    Err("package not found")
}

pub fn search<'a> (contents: &'a str, query: &str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for i in contents.lines() {
        if i.contains(query) {
            results.push(i);
        }
    }
    results
} 

#[cfg(test)]
mod tests {


    #[test]
    fn test_input() {
    }
}

