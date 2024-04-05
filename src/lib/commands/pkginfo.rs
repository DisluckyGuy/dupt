use std::fs;

use tar::Archive;


use crate::tools::{packages, paths::{self, get_root_path}};

use super::Command;
pub struct PkgInfo {
    name: String,
}

impl Command for PkgInfo {
    fn help(&self) {
        todo!()
    }

    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        if packages::search_installed(&self.name).is_ok() {
            let info =
                fs::read_to_string(format!("{}/.dupt/installed/{}", get_root_path(), self.name))?;
            println!("{}", info);
            return Ok(());
        }
        packages::search_package(&self.name)?;
        packages::get_file(
            &format!("{}.tar.gz", &self.name),
            "dupt-repo-main",
            format!("{}/.dupt/archives", paths::get_root_path()),
            "main",
        )?;
        let tar_file = fs::File::open(format!(
            "{}/.dupt/archives/{}.tar.gz",
            paths::get_root_path(),
            self.name
        ))?;
        let tar = flate2::read::GzDecoder::new(&tar_file);
        let mut archive = Archive::new(tar);
        archive.unpack(format!("{}/.dupt/archives", get_root_path()))?;
        let info = fs::read_to_string(format!(
            "{}/.dupt/archives/{}/PKGINFO.conf",
            get_root_path(),
            self.name
        ))?;
        println!("{}", info);
        packages::clear_archives(&"ghostknight".to_string())?;
        Ok(())
    }

    fn set_from_args(&mut self, args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() == 0 {
            return Err("not enough arguments")?;
        }
        self.name = args[0].to_string();
        Ok(())
    }
}

impl Default for PkgInfo {
    fn default() -> Self {
        Self {
            name: String::from("help"),
        }
    }
}
