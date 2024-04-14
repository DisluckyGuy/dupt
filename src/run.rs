use std::error::Error;
use libdupt::commands::{install::Install, pkginfo::PkgInfo, remove::Remove, run::Run, search::Search, update::Update};
use libdupt::config::Config;
use libdupt::tools::paths::check_root_path;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    check_root_path();
    let args = &config.arguments;
    if config.process == "install" {
        let command = Install::from_args(args)?;
        command.run()?;
    } else if config.process == "run" {
        let command = Run::from_args(args)?;
        command.run()?;
    } else if config.process == "remove" {
        let command = Remove::from_args(args)?;
        command.run()?;
    } else if config.process == "search" {
        let command = Search::from_args(args)?;
        command.run()?;
    } else if config.process == "pkginfo" {
        let command = PkgInfo::from_args(args)?;
        command.run()?;
    } else if config.process == "update" {
        let command = Update::from_args(args)?;
        command.run()?;
    } else {
        Err("invalid command")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    
}