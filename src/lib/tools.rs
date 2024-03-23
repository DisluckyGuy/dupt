use std::{
    env,
    error::Error,
    io::{self, Write},
};


pub fn get_project_dir() -> String {
    let current_exec = env::current_exe().unwrap();
    let project_dir: Vec<&str> = current_exec.to_str().unwrap().rsplitn(4, "/").collect();
    String::from(project_dir[project_dir.len() - 1])
}

pub fn list_path() -> String {
    String::from(format!("{}/packages/list.config", get_project_dir()))
}

pub fn installed_path() -> String {
    String::from(format!("{}/packages/installed.config", get_project_dir()))
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