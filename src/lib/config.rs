pub struct Config {
    pub process: String,
    pub arguments: Vec<String>,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() == 1 {
            println!("ugs package manager for niche software!");
            std::process::exit(0);
        }
        let process = args[1].clone();
        if process == "install" {
            if args.len() == 2 {
                return Err("two few arguments for install process");
            }
        }
        let arguments = if args.len() > 2 {
            args.split_at(2).1.to_vec()
        } else {
            Vec::new()
        };
        Ok(Config { process, arguments })
    }
}