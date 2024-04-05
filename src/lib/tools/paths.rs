use std::{env, fs, io::{self, Write}, process};

pub fn list_path() -> String {
    String::from(format!("{}/sources/list.config", get_project_dir()))
}

pub fn installed_path() -> String {
    String::from(format!("{}/sources/installed.config", get_project_dir()))
}

pub fn get_project_dir() -> String {
    let current_exec = env::current_exe().unwrap();
    let project_dir: Vec<&str> = current_exec
        .to_str()
        .unwrap()
        .split_inclusive("dupt")
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

pub fn check_root_path() {
    if fs::File::open(format!("{}/configs/configs.conf", get_project_dir())).is_ok() {
        return;
    }
    let config_file =
        fs::File::create(format!("{}/configs/configs.conf", get_project_dir())).unwrap();
    let mut writer = io::BufWriter::new(config_file);
    let _user =
        String::from_utf8(process::Command::new("whoami").output().unwrap().stdout).unwrap();
    let user_name = _user.trim();
    writer
        .write(format!("root_path: /home/{}\n", user_name).as_bytes())
        .unwrap();
}