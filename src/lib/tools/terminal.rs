use std::{error::Error, io::{self, Write}};

pub fn print_blue(text: &str) {
    println!("{}", ansi_term::Color::Blue.paint(text))
}

pub fn print_red(text: &str) {
    println!("{}", ansi_term::Color::Red.paint(text))
}

pub fn print_green(text: &str) {
    println!("{}", ansi_term::Color::Green.paint(text))
}

pub fn confirm(text: &str) -> Result<bool, Box<dyn Error>> {
    print!("{}", text);
    io::stdout().flush()?;
    let mut confirm = String::new();
    std::io::stdin().read_line(&mut confirm)?;
    confirm = String::from(confirm.trim());

    if confirm == "n" {
        println!("aborting...");
        return Ok(false);
    } else if confirm != "y" {
        return Err("invalid value. aborting...")?;
    }

    Ok(true)
}