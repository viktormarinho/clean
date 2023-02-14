use clap::Parser;
use std::process::{Command, Stdio};
use walkdir::{WalkDir, DirEntry};
use std::fs;

#[derive(Parser, Debug)]
#[command(
    author = "viktormarinho", 
    version = "0.0.1", 
    about = "Hammer is a no-config cli tool for running concurrent tasks with monorepo support", 
    long_about = None)]
struct Args {
    project: String,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn main() {
    let args  = Args::parse();

    let echo_child = Command::new("echo")
    .arg("Standard outputting ^^!")
    .stdout(Stdio::piped())
    .output()
    .expect("Failed to start echo process");

    let is_ignored = |path: &str| {
        let output = Command::new("git")
                                .arg("check-ignore")
                                .arg(format!("{}", path))
                                .output()
                                .expect("Failed to execute command");

        if output.stdout.len() > 0 {
            return true
        }

        false
    };

    // Note that `echo_child` is moved here, but we won't be needing
    // `echo_child` anymore
    let message = String::from_utf8_lossy(&echo_child.stdout);

    for entry in WalkDir::new(".")
    .min_depth(1)
    .into_iter()
    .filter_entry(|f| { !is_ignored(f.file_name().to_str().unwrap()) && !is_hidden(&f) })
    .filter_map(|f| { f.ok() })
    .filter(|f| {
        if let Ok(meta) = f.metadata() {
            return meta.is_file();
        }
        false
    }) {
        println!("{}",  entry.path().display());
    }

    println!("Process wrote: {}", message);
}
