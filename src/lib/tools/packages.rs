use std::{collections, error::Error, fs, process};

use super::{containers, paths::{self, get_project_dir}};

pub fn search_package(name: &str) -> Result<(), Box<dyn Error>> {
    let repo_dir = fs::read_dir(format!("{}/sources/repositories", paths::get_project_dir()))?;
    for i in repo_dir {
        let repo_file = fs::read_to_string(i.unwrap().path())?;
        for j in repo_file.lines() {
        if &j[0..4] == "    " || j.trim().is_empty() {
            continue;
        }
        if j.trim() == name {
            return Ok(());
        }
    }
    }
    

    Err("Package not found")?
}

pub fn search_installed(name: &str) -> Result<(), Box<dyn Error>> {
    let entries = fs::read_dir(&format!("{}/.dupt/installed", paths::get_root_path()))?;
    for i in entries {
        if i.unwrap().file_name() == name.trim() {
            return Ok(());
        }
    }
    
    Err("Package not found")?
}

pub fn get_dependency_count() -> collections::HashMap<String, i32>{
    let mut dependency_list: collections::HashMap<String, i32> = collections::HashMap::new();
    let installed_dir = fs::read_dir(&format!("{}/.dupt/installed", paths::get_root_path())).unwrap();
    for i in installed_dir {
        let entry = fs::read_to_string(i.unwrap().path()).unwrap();
        let mut in_dependencies = false;
        for j in entry.lines() {
            if j.trim() == "]" && in_dependencies {
                break;
            } else if j.trim() == "]" && !in_dependencies {
                continue;
            }
            if &j[0..4] == "    " && !in_dependencies {
                continue;
            }
            if &j[0..4] == "    " && in_dependencies {
                if dependency_list.contains_key(j.trim()) {
                    *dependency_list.get_mut(j.trim()).unwrap() += 1;
                    continue;
                }
                dependency_list.insert(j.trim().into(), 1);
                continue;
            }
            let key = j.split_once(":").unwrap().0.trim();
            if key == "dependencies" {
                in_dependencies = true;
            }
        }
    }
    dependency_list
}

pub fn get_dependencies(name: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut dependencies: Vec<String> = Vec::new();
    let pkg_file = fs::read_to_string(format!("{}/.dupt/installed/{}", paths::get_root_path(), name))?;
    let mut in_dependencies = false;
    for i in pkg_file.lines() {
        if i.trim() == "]" && in_dependencies {
            break;
        } else if i.trim() == "]" && !in_dependencies {
            continue;
        }
        if &i[0..4] == "    " && !in_dependencies {
            continue;
        }
        if &i[0..4] == "    " && in_dependencies {
            dependencies.push(i.trim().into());
            continue;
        }
        let key = i.split_once(":").unwrap().0.trim();
        if key == "dependencies" {
            in_dependencies = true;
        }
    }
    Ok(dependencies)
}

pub fn clear_archives(name: &String) -> Result<(), Box<dyn Error>> {
    containers::run_distrobox_command(
        &format!(
            "rm {0}/.dupt/archives/{1}.tar.gz {0}/.dupt/archives/{1} -r",
            paths::get_root_path(),
            name
        ),
        false,
    )?;
    Ok(())
}

pub fn get_make_dependencies(name: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut dependencies: Vec<String> = Vec::new();
    let pkg_file = fs::read_to_string(format!("{}/.dupt/installed/{}", paths::get_root_path(), name))?;
    let mut in_dependencies = false;
    for i in pkg_file.lines() {
        if i.trim() == "]" && in_dependencies {
            break;
        } else if i.trim() == "]" && !in_dependencies {
            continue;
        }
        if &i[0..4] == "    " && !in_dependencies {
            continue;
        }
        if &i[0..4] == "    " && in_dependencies {
            dependencies.push(i.trim().into());
            continue;
        }
        let key = i.split_once(":").unwrap().0.trim();
        if key == "make_dependencies" {
            in_dependencies = true;
        }
    }
    Ok(dependencies)
}

pub fn get_unused_dependencies(name: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut unused_dependencies: Vec<String> = Vec::new();
    let list = get_dependency_count();
    let pkg_dep = get_dependencies(name)?;

    for i in list.keys() {
        for j in &pkg_dep {
            if j != i {
                continue;
            }
            if list[i] > 1 {
                continue;
            }
            unused_dependencies.push(i.trim().into());
        }
    }

    Ok(unused_dependencies)
}

pub fn get_file(
    name: &String,
    output: &String,
    repo: &str,
    path: String,
) -> Result<(), Box<dyn Error>> {
    let mut repo_link = &String::new();
    let repositries = get_repos();
    for i in repositries.keys() {
        if i == &repo {
            repo_link = repositries.get(i).unwrap();
        }
    }
    println!("running curl");
    let pkg_loc = repo_link.split_once("||").unwrap();
    let _curl = process::Command::new("curl")
        .current_dir(path)
        .arg("-o")
        .arg(format!("{}", output))
        .arg(format!("{}{}{}",  pkg_loc.0, name, pkg_loc.1))
        .arg("-l")
        .spawn()?
        .wait()?;
    Ok(())
}

pub fn get_repos() -> collections::HashMap<String, String> {
    let mut repos: collections::HashMap<String, String> = collections::HashMap::new();
    let source_file = fs::read_to_string(format!("{}/sources/sources.conf", get_project_dir())).unwrap();
    for i in source_file.lines() {
        if i.trim().is_empty() {
            continue;
        }
        let line = i.split_once(":").unwrap();
        let name = line.0.trim().to_string();
        let link = line.1.trim().to_string();
        repos.insert(name, link);
    }
    println!("{:?}", repos);
    repos
}