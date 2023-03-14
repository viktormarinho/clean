use colored::Colorize;

pub fn print(msg: &str) {
    println!("{} {}", "[EINER]".purple(), msg.white());
}