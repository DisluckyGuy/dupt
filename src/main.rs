use std::env;
use std::process;

use ugs::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let configs = Config::new(args).unwrap_or_else(|err| {
        println!("program aborted for: {}", err);
        process::exit(1);
    });

    if let Err(e) = ugs::run(configs) {
        println!("running failed with error message: {e}");
        process::exit(1);
    }
    
}
