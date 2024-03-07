use std::error::Error;
pub mod config;
pub mod package;
pub mod tools;
use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let package_list = std::fs::read_to_string(format!("{}/packages/list.config",tools::get_project_dir()))?;
    let installed_list = std::fs::read_to_string(format!("{}/packages/installed.config", tools::get_project_dir()))?;

    if config.process == "install" {
        let mut package_names: Vec<&String> = Vec::new();
        for i in &config.arguments {
            if &i[0..1] == "-" {
                continue;
            }
            if tools::search_package(&installed_list, &i).is_ok() {
                Err(format!("package \"{}\" already installed", i))?
            }
            tools::search_package(&package_list, &i)?;
            println!("found package \"{}\"", i);
            package_names.push(i);
        }

        println!();
        println!("packages to install:");
        println!();

        for i in package_names {println!("{}", i);}
        println!();
        
        if !config.arguments.contains(&String::from("-y")) {
            if !tools::confirm()? {
                println!("aborting...");
                return Ok(());
            }
        }

        for i in &config.arguments {
            if &i[0..1] == "-" {
                continue;
            }
            let package = tools::search_package(&package_list, &i)?;
            package.install()?;
        }
         
        return Ok(());
    } else if config.process == "remove" {
        let mut package_names: Vec<&String> = Vec::new();
        for i in &config.arguments {
            if &i[0..1] == "-" {
                continue;
            }
            if tools::search_package(&installed_list, &i).is_err() {
                Err(format!("package \"{}\" not installed", i))?
            }
            println!("found package \"{}\"", i);
            package_names.push(i);
        }

        println!();
        println!("packages to remove:");
        println!();

        for i in package_names {println!("{}", i);}
        println!();
        
        if !config.arguments.contains(&String::from("-y")) {
            if !tools::confirm()? {
                println!("aborting...");
                return Ok(());
            }
        }

        for i in &config.arguments {
            if &i[0..1] == "-" {
                continue;
            }
            let package = tools::search_package(&installed_list, &i)?;
            package.remove()?;
        }
         
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
        let package = tools::search_package(&installed_list,&config.arguments[0])?;
        package.run()?;
        return Ok(());
    } else if config.process == "pkginfo" {
        let package = tools::search_package(&package_list,&config.arguments[0])?;
        println!("{}", package.pkginfo()?);
        return Ok(());
    }
    Err("invalid command")?
}
