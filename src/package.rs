use std::env;
use std::env::current_dir;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

pub struct Package {
    pub name: String,
    pub zip: String,
    pub description: String,
    pub install: Vec<String>,
    pub runnable: bool,
    pub run: Vec<String>
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
            println!("running command: {}", &program);
            let _cmd = process::Command::new(&program)
                .args(args)
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
            .arg(format!("{}/packages/programs/{}",get_project_dir(), self.zip))
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

        env::set_current_dir(format!("{}/packages/programs/{}",get_project_dir(),self.zip))?;
        println!("{}", current_dir().unwrap().to_str().unwrap());

        let commands = &self.run;

        for i in commands {
            let command: Vec<&str> = i.split_whitespace().collect();
            let program = command.split_at(1).0.concat();
            let args = command.split_at(1).1.to_vec();

            println!("running command: {}", program);

            if program == "cd" {
                let current_dir =  env::current_dir().unwrap();
                env::set_current_dir(format!("{}/{}", current_dir.to_str().unwrap(), args.concat()))?;
                println!("{} successful", &program);
                continue;
            }
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
    
    pub fn pkginfo<'a> (&'a self) -> Result<String, Box<dyn Error>> {
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
