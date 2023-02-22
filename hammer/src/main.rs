use clap::Parser;
use std::process::{Command, Stdio};
use walkdir::{WalkDir, DirEntry};
use std::path::Path;
use std::io::{BufRead, BufReader,Write};
use std::io;

#[derive(Parser, Debug)]
#[command(
    author = "viktormarinho", 
    version = "0.0.1", 
    about = "Hammer is a no-config cli tool for running concurrent tasks with monorepo support", 
    long_about = None)]
struct Args {
    command: String,
}

fn start_npm_process<T: 'static + Send + Fn(&str)>(process_dir: &Path, cmd: &String, cb: T) {
    let child = Command::new("npm")
            .current_dir(process_dir)
            .arg("run")
            .arg(format!("hammer:{}", cmd))
            .stdout(Stdio::piped())
            .spawn()
            .expect(&format!("Could not start child process on directory {}", process_dir.display()));
    
    tokio::spawn(async move {
        let mut f = BufReader::new(child.stdout.expect("Could not retrieve child std output"));
        loop {
            let mut buf = String::new();
            match f.read_line(&mut buf) {
                Ok(_) => {
                    cb(buf.as_str());
                },
                Err(e) => println!("child err: {:?}", e)
            }
        }
    });
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn get_package_json(file_path: &str) -> serde_json::Value {
    let file_data = std::fs::read_to_string(file_path).expect(
        &format!("Could not read file {file_path}")
    );
    serde_json::from_str(&file_data).expect(
        &format!("Could not parse json file {file_path}")
    )
}

#[tokio::main]
async fn main() {
    let args  = Args::parse();

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
    })
    .filter(|f| {
        f.path().to_str().unwrap().ends_with("package.json")   
    })
    .map(|f| {
        (str::replace(f.path().to_str().unwrap(), "package.json", ""), 
        get_package_json(f.path().to_str().unwrap()))
    }) {
        let process_dir = Path::new(&entry.0);
        let project_package_json_path = String::from(format!("{}package.json", process_dir.display()));
        entry.1.get("scripts").expect(
            &format!("Could not find the 'scripts' block at the file {}", project_package_json_path)
        ).get(format!("hammer:{}", args.command)).expect(
            &format!("Could not find the desired script hammer:'{}' at the file {}", 
            args.command, 
            project_package_json_path));

        let process_name = entry.1.get("name").expect(
            &format!("Could not find project name at file {}", project_package_json_path)
        );

        let process_name = match process_name {
            serde_json::Value::String(name) => name.clone(),
            _ => panic!("Project name at file {} was not a string", project_package_json_path)
        };

        start_npm_process(process_dir, &args.command, move |msg| {
            if msg.len() > 0 {
                print!("{}: {}", process_name, msg);
                io::stdout().flush().expect("Could not flush stdout");
            }
        });
    }
}
