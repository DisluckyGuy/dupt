use std::{env, error::Error, fs, process::{self, exit}};

use super::paths;
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

pub fn make_dupt_folder() -> Result<(), Box<dyn Error>> {
    let config_file = fs::read_to_string(format!("{}/configs/configs.conf", paths::get_project_dir()))?;

    if env::consts::OS == "linux" {
        println!("chowning");
        let _chown = process::Command::new("chmod")
            .arg("+x")
            .arg(format!("{}/scripts/*", paths::get_project_dir()));
    }

    println!("checking presence of project root");

    run_distrobox_command(
        &format!(
            "sh {}/scripts/mkdupt.sh {}",
            paths::get_project_dir(),
            paths::get_root_path()
        ),
        false,
    )?;

    println!("checking config file");

    let fedora_config = String::from_utf8(
        run_distrobox_command(
            &format!("cat {}/.dupt/configs/configs.conf", paths::get_root_path()),
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
                paths::get_root_path()
            ),
            false,
        )?;
    }
    Ok(())
}


pub fn run_distrobox_command(args: &str, spawn: bool) -> Result<process::Output, Box<dyn Error>> {
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
