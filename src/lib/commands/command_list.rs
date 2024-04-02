use std::collections::HashMap;
use super::install;

use super::run;
use super::search;
use super::Command;

pub struct CommandList {
    pub list: HashMap<String, Box<dyn Command>>
}

impl Default for CommandList{
    fn default() -> Self {
        let mut list = HashMap::new() as HashMap<String, Box<dyn Command>>;
        let install = install::Install::default();
        let search = search::Search::default();
        let run = run::Run::default();
        list.insert(String::from("install"), Box::new(install));
        list.insert(String::from("search"), Box::new(search));
        list.insert(String::from("run"), Box::new(run));
        Self {list}
    }
}