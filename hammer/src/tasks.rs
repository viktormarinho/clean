use colored::Colorize;
use std::collections::HashMap;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, io};

use crate::errors::BeautifulErrors;
use crate::npm_process::NpmProcessContext;

fn handle_process_stdout(process_name: &String, msg: &str) {
    if msg.len() > 0 {
        print!("{}: {}", process_name.blue().bold(), msg);
        io::stdout().flush().expect("Could not flush stdout");
    }
}

pub fn start_npm_process(context: NpmProcessContext) {
    let path = Path::new(&context.dir);
    let mut root_env: HashMap<String, String> = env::vars().collect();

    if let Some(envs) = context.args.env {
        envs.iter().for_each(|var| {
            let key_value: Vec<&str> = var.split(":").collect();
            let key = key_value.get(0).expect_or_err(
                &format!("Invalid environment variable provided: {} - Usage: hammer dev -e node_env:development", var)
            );

            let val = key_value.get(1).expect_or_err(
                &format!("Invalid environment variable provided: {} - Usage: hammer dev -e node_env:development", var)
            );

            root_env.insert(key.to_string(), val.to_string());
        });
    }
    let mut child = Command::new("npm")
        .current_dir(path)
        .envs(&root_env)
        .arg("run")
        .arg(context.args.script)
        .stdout(Stdio::piped())
        .spawn()
        .expect(&format!(
            "Could not start child process on directory {}",
            path.display()
        ));

    tokio::spawn(async move {
        let mut f = BufReader::new(
            child
                .stdout
                .take()
                .expect("Could not retrieve child std output"),
        );
        loop {
            let mut buf = String::new();
            match f.read_line(&mut buf) {
                Ok(_) => {
                    handle_process_stdout(&context.name, buf.as_str());
                }
                Err(e) => println!("child err: {:?}", e),
            }
            if let Ok(status) = child.try_wait() {
                if let Some(_status) = status {
                    break;
                }
            }
        }
    });
}
