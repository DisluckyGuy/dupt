use std::{collections, error::Error, fs, process};

use super::{containers, paths};

pub fn search_package(name: &str) -> Result<(), Box<dyn Error>> {
    for i in std::fs::read_to_string(paths::list_path())?.lines() {
        if &i[0..4] == "    " || i.trim().is_empty() {
            continue;
        }
        if i.trim() == name {
            return Ok(());
        }
    }

    Err("Package not found")?
}

pub fn search_installed(name: &str) -> Result<(), Box<dyn Error>> {
    let _ls =
        containers::run_distrobox_command(&format!("ls {}/.dupt/installed", paths::get_root_path()), false)?.stdout;
    let file_list = String::from_utf8(_ls)?;
    let installed_files: Vec<&str> = file_list.split_whitespace().collect();

    if installed_files.contains(&name) {
        return Ok(());
    }

    for i in std::fs::read_to_string(paths::installed_path())?.lines() {
        if &i[0..4] == "    " || i.trim().is_empty() {
            continue;
        }
        if i.trim() == name {
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
    repo: &str,
    path: String,
    branch: &str,
) -> Result<(), Box<dyn Error>> {
    let mut repo_link = String::new();
    for i in fs::read_to_string(format!("{}/sources/repos.conf", paths::get_project_dir()))?.lines() {
        let line = i.split_once(":").unwrap();
        let name = &line.0.trim();
        let link = &line.1.trim();
        println!("line: {:?}", line);
        println!("name: {}", name);
        println!("link: {}", link);
        if name == &repo {
            repo_link = link.to_string();
        }
    }
    println!("running curl");
    println!("{}/{}/raw?ref={}", repo_link, name, branch);
    let _curl = process::Command::new("curl")
        .current_dir(path)
        .arg("-o")
        .arg(format!("{}", name))
        .arg(format!("{}/{}/raw?ref={}", repo_link, name, branch))
        .arg("-l")
        .spawn()?
        .wait()?;
    Ok(())
}