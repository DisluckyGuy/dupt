use std::env;
use std::process;
use libdupt::config;
use ansi_term::Color;
mod run;
fn main() {
    
    let args: Vec<String> = env::args().collect();
    let configs = config::Config::new(args).unwrap();
    if let Err(e) = run::run(configs) {
        println!("{}", Color::Red.paint(format!("running failed with error message: {e}")));
        process::exit(1);
    };    
}
