pub mod search;

use std::error::Error;

use crate::config::Config;

pub trait Command {
    fn help(&self);
    fn run(&self) -> Result<(), Box<dyn Error>>;
    fn build<'a> (&self, config: &Config) -> Result<Box<dyn Command>, Box<dyn Error>>;
}