use std::env;
use std::process;
use libdupt::config;
use ansi_term::Color;
fn main() {
    let args: Vec<String> = env::args().collect();
    let params = &args.split_at(2).1;
    let process = args[1].clone();
    let configs = config::Config {process, arguments: params.to_vec()};

    if let Err(e) = libdupt::run(configs) {
        println!("{}", Color::Red.paint(format!("running failed with error message: {e}")));
        process::exit(1);
    };    
}
