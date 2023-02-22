use walkdir::DirEntry;
use std::process::Command;

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

pub fn is_ignored(path: &str) -> bool {
    let output = Command::new("git")
                            .arg("check-ignore")
                            .arg(format!("{}", path))
                            .output()
                            .expect("Failed to execute command - do you have git installed?");

    output.stdout.len() > 0 
}