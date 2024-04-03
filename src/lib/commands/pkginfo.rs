use std::fs;

use tar::Archive;

use crate::tools::{self, get_root_path, search_package};

use super::Command;
pub struct PkgInfo {
    name: String,
}

impl Command for PkgInfo {
    fn help(&self) {
        todo!()
    }

    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        if tools::search_installed(&self.name).is_ok() {
            let info =
                fs::read_to_string(format!("{}/.dupt/installed/{}", get_root_path(), self.name))?;
            println!("{}", info);
            return Ok(());
        }
        search_package(&self.name)?;
        tools::get_file(
            &format!("{}.tar.gz", &self.name),
            "dupt-repo-main",
            format!("{}/.dupt/archives", tools::get_root_path()),
            "main",
        )?;
        let tar_file = fs::File::open(format!(
            "{}/.dupt/archives/{}.tar.gz",
            tools::get_root_path(),
            self.name
        ))?;
        let tar = flate2::read::GzDecoder::new(&tar_file);
        let mut archive = Archive::new(tar);
        archive.unpack(format!("{}/.dupt/archives", tools::get_root_path()))?;
        let info = fs::read_to_string(format!(
            "{}/.dupt/archives/{}/PKGINFO.conf",
            get_root_path(),
            self.name
        ))?;
        println!("{}", info);
        tools::clear_archives(&"ghostknight".to_string())?;
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
