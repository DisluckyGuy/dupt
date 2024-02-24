use std::error::Error;
pub mod config;
pub mod package;
use config::Config;
use package::Package;
use package::get_project_dir;

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
