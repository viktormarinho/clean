use colored::Colorize;

pub fn print(msg: &str) {
    println!("{} {}", "[EINER]".purple(), msg.white());
}

pub fn print_err(msg: &str) {
    println!("{} {}", "[EINER]".purple(), msg.red());
}