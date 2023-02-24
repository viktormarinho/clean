use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::io;
use colored::Colorize;
use std::io::Write;

use crate::npm_process::NpmProcessContext;

fn handle_process_stdout(process_name: &String, msg: &str) {
    if msg.len() > 0 {
        print!("{}: {}", process_name.blue().bold(), msg);
        io::stdout().flush().expect("Could not flush stdout");
    }
}

pub fn start_npm_process(context: NpmProcessContext) {
    let path = Path::new(&context.dir);
    let mut child = Command::new("npm")
            .current_dir(path)
            .arg("run")
            .arg(context.script)
            .stdout(Stdio::piped())
            .spawn()
            .expect(&format!("Could not start child process on directory {}", path.display()));
    
    tokio::spawn(async move {
        let mut f = BufReader::new(child.stdout.take().expect("Could not retrieve child std output"));
        loop {
            let mut buf = String::new();
            match f.read_line(&mut buf) {
                Ok(_) => {
                    handle_process_stdout(&context.name, buf.as_str());
                },
                Err(e) => println!("child err: {:?}", e)
            }
            if let Ok(status) = child.try_wait() {
                if let Some(_status) = status {
                    break
                }
            }
        }
    });
}
