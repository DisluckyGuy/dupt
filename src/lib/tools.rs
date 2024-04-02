use std::{
    env,
    error::Error,
    fs,
    io::{self, Write},
    process::{self, exit, Output},
};

use ansi_term;


pub fn run_distrobox_command(args: &str, spawn: bool) -> Result<Output, Box<dyn Error>> {
    let command_vec: Vec<&str> = args.split_whitespace().collect();
    let mut binding = process::Command::new("distrobox");
    let _command = binding
        .arg("enter")
        .arg("dupt-fedora")
        .arg("--")
        .args(command_vec);
    if spawn {
        let _spawn = _command.spawn()?.wait()?;
    }
    if !_command.output()?.status.success() {
        let err_msg = String::from_utf8(_command.output()?.stderr.to_vec())?;
        Err(format!(
            "container command failed with error message: {}",
            err_msg
        ))?;
    }
    Ok(_command.output()?)
}

pub fn search_package(name: &str) -> Result<(), Box<dyn Error>> {
    for i in std::fs::read_to_string(list_path())?.lines() {
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
    let _ls = run_distrobox_command(&format!("ls {}/.dupt/installed", get_root_path()), false)?.stdout;
    let file_list = String::from_utf8(_ls)?;
    let installed_files: Vec<&str> = file_list.split_whitespace().collect();
    
    if installed_files.contains(&name) {
        return Ok(());
    }

    for i in std::fs::read_to_string(installed_path())?.lines() {
        if &i[0..4] == "    " || i.trim().is_empty() {
            continue;
        }
        if i.trim() == name {
            return Ok(());
        }
    }

    Err("Package not found")?
}

pub fn get_file(name: &String, repo: &str, path: String, branch: &str) -> Result<(), Box<dyn Error>> {
    let mut repo_link = String::new();
    for i in fs::read_to_string(format!("{}/sources/repos.conf", get_project_dir()))?.lines() {
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

pub fn make_dupt_folder() -> Result<(), Box<dyn Error>> {
    let config_file = fs::read_to_string(format!("{}/configs/configs.conf", get_project_dir()))?;

    if env::consts::OS == "linux" {
        println!("chowning");
        let _chown = process::Command::new("chmod")
            .arg("+x")
            .arg(format!("{}/scripts/*", get_project_dir()));
    }

    println!("checking presence of project root");

    run_distrobox_command(
        &format!(
            "sh {}/scripts/mkdupt.sh {}",
            get_project_dir(),
            get_root_path()
        ),
        false,
    )?;

    println!("checking config file");

    let fedora_config = String::from_utf8(
        run_distrobox_command(
            &format!("cat {}/.dupt/configs/configs.conf", get_root_path()),
            false,
        )?
        .stdout,
    )?;

    if fedora_config.trim() != config_file.trim() {
        println!("entering configs");
        run_distrobox_command(
            &format!(
                "echo {} > {}/.dupt/configs/configs.conf",
                config_file,
                get_root_path()
            ),
            false,
        )?;
    }
    Ok(())
}

pub fn print_blue(text: &str) {
    println!("{}", ansi_term::Color::Blue.paint(text))
}

pub fn print_red(text: &str) {
    println!("{}", ansi_term::Color::Red.paint(text))
}

pub fn print_green(text: &str) {
    println!("{}", ansi_term::Color::Green.paint(text))
}

pub fn get_project_dir() -> String {
    let current_exec = env::current_exe().unwrap();
    let project_dir: Vec<&str> = current_exec
        .to_str()
        .unwrap()
        .split_inclusive("ugs")
        .collect();
    String::from(project_dir[0])
}

pub fn get_root_path() -> String {
    let config =
        std::fs::read_to_string(format!("{}/configs/configs.conf", get_project_dir())).unwrap();
    let mut root_path = String::new();
    for i in config.lines() {
        let key = i.split_once(":").expect("unable to split").0;
        let value = i.split_once(":").expect("unable to split").1;
        if key == "root_path" {
            root_path = String::from(value.trim());
        }
    }
    root_path
}

pub fn get_fedora_image() -> String {
    String::from("fedora:40")
}

pub fn check_toolbox_env() -> Result<(), Box<dyn Error>> {
    println!("listing containers");

    let mut _list_containers = process::Command::new("distrobox").arg("list").output()?;

    if !_list_containers.status.success() {
        println!(
            "distrobox required dependency not met, please install it with your package manager"
        );
        exit(1);
    }

    println!("chacking container prescense");

    let output = String::from_utf8(_list_containers.stdout)?;
    println!("{}", output);
    if !output.contains("dupt-fedora") {
        let _create_container = process::Command::new("distrobox")
            .arg("create")
            .arg("dupt-fedora")
            .arg("--image")
            .arg(get_fedora_image())
            .arg("-Y")
            .spawn()?
            .wait();
    }

    println!("updating fedora container");
    let _update_fedora = run_distrobox_command("sudo dnf update -y", true)?;

    Ok(())
}

pub fn clear_archives(name: &String) -> Result<(), Box<dyn Error>> {
    run_distrobox_command(&format!("rm {0}/.dupt/archives{1}.tar.gz {0}/.dupt/archives{1} -r", get_root_path(), name), false)?;
    Ok(())
}

pub fn list_path() -> String {
    String::from(format!("{}/sources/list.config", get_project_dir()))
}

pub fn installed_path() -> String {
    String::from(format!("{}/sources/installed.config", get_project_dir()))
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
