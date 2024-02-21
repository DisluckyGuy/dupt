use std::error::Error;
use std::io::{self, Write};
use std::{env, fs, process};

pub struct Config {
    process: String,
    arguments: Vec<String>,
}

pub struct Package {
    name: String,
    zip: String,
    description: String,
    install: Vec<String>,
    runnable: bool,
    run: Vec<String>
}

pub fn get_project_dir() -> String {
    let current_exec = env::current_exe().unwrap();
    let project_dir: Vec<&str> = current_exec.to_str().unwrap().rsplitn(4, "/").collect();
    String::from(project_dir[project_dir.len() - 1])
}

impl Package {
    pub fn install(self) -> Result<(), Box<dyn Error>> {
        let working_dir = env::current_dir().unwrap().display().to_string();

        env::set_current_dir(get_project_dir())?;

        println!("package found!");
        println!("packages to install: ");
        println!();
        println!("{}", self.name);
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

        for i in self.install {
            let command: Vec<&str> = i.split_whitespace().collect();
            let program = command.split_at(1).0.concat();
            let args = command.split_at(1).1.to_vec();
            println!("running command: {}", args[0]);
            let _cmd = process::Command::new(&program)
                .args(args.split_at(1).1)
                .output()?;
            if !_cmd.status.success() {
                return Err(format!("{} failed", &program))?;
            }
            println!("{} successful", &program);
        }

        println!("installed package: {}", self.name);

        let mut fb = fs::OpenOptions::new()
            .append(true)
            .write(true)
            .open(format!("{}/packages/installed.config",get_project_dir()))?;
        let line = format!("{}: {}", self.name, self.zip);
        writeln!(fb, "{line}")?;
        env::set_current_dir(working_dir)?;
        Ok(())
    }

    pub fn remove(&self) -> Result<(), Box<dyn Error>> {
        let working_dir = env::current_dir().unwrap().display().to_string();

        env::set_current_dir(get_project_dir())?;

        println!("package found!");
        println!("packages to remove: ");
        println!();
        println!("{}", self.name);
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
            .arg(format!("{}/packages/programs/{}/{}",get_project_dir(), self.zip, self.zip))
            .arg("--r")
            .arg("--f")
            .output()?;

        if !_rmdir.status.success() {
            Err("removing failed")?;
        }

        println!("removing..");
        println!();
        println!("removed package: {}", self.name);

        let install_list = std::fs::read_to_string(format!("{}/packages/installed.config", get_project_dir()))?;
        std::fs::write(format!("{}/packages/installed.config", get_project_dir()), "")?;
        let mut fb = fs::OpenOptions::new()
            .append(true)
            .write(true)
            .open(format!("{}/packages/installed.config", get_project_dir()))?;
        for i in install_list.lines() {
            if i.trim_end() == format!("{}: {}", self.name, self.zip) {
                continue;
            } else {
                writeln!(fb, "{i}")?;
            }
        }
        env::set_current_dir(working_dir)?;
        Ok(())
    }
    pub fn run(&self) -> Result<(), Box<dyn Error>> {

        println!("running..");

        if !self.runnable {
            return Err("package not runnable")?;
        }

        let working_dir = env::current_dir().unwrap().display().to_string();

        env::set_current_dir(format!("{}/packages/programs/{}/{}",get_project_dir(),self.zip, self.zip))?;

        let commands = &self.run;

        for i in commands {
            let command: Vec<&str> = i.split_whitespace().collect();
            let program = command.split_at(1).0.concat();
            let args = command.split_at(1).1.to_vec();
            println!("running command: {}", program);
            let _cmd = process::Command::new(&program)
            .args(args)
            .output()?;

            if !_cmd.status.success() {
                return Err(format!("{} failed", &program))?;
            }

            println!("{} successful", &program);
        }

        env::set_current_dir(working_dir)?;

        Ok(())
    }
    
    fn pkginfo<'a> (&'a self) -> Result<String, Box<dyn Error>> {
        let mut info = String::new();
        let mut in_block = false;
        let pkg_config = fs::read_to_string(format!("{}/packages/programs/{}/{}.config", get_project_dir(), self.zip, self.name))?;
        for i in pkg_config.lines() {
            if in_block {
                continue;
            }
            if i.contains("[") {
                in_block = true;
                continue;
            } else if i.contains("]") && in_block {
                in_block = false;
                continue;
            }
            info += i;
            info += "\n";
        }
        info = info.trim_end().to_string();
        Ok(info)
    }

}

impl Config {
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
        let arguments = if args.len() > 2 {
            args.split_at(2).1.to_vec()
        } else {
            Vec::new()
        };
        Ok(Config { process, arguments })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let package_list = std::fs::read_to_string(format!("{}/packages/list.config",get_project_dir()))?;
    let installed_list = std::fs::read_to_string(format!("{}/packages/installed.config", get_project_dir()))?;
    if config.process == "install" {
        if search_package(&installed_list, &config.arguments[0]).is_ok() {
            Err("package already installed")?
        }
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
            println!("{}", &i[0..index]);
        }
        return Ok(());
    } else if config.process == "listinstalled" {
        for i in installed_list.lines() {
            let index = i.find(":").unwrap();
            println!("{}", &i[0..index]);
        }
        return Ok(());
    } else if config.process ==  "run" {
        let package = search_package(&installed_list,&config.arguments[0])?;
        package.run()?;
        return Ok(());
    } else if config.process == "pkginfo" {
        let package = search_package(&package_list,&config.arguments[0])?;
        println!("{}", package.pkginfo()?);
        return Ok(());
    }
    Err("invalid command")?
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
                "{}/packages/programs/{}/{}.config",
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

pub fn search<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
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
    fn test_input() {}
}
