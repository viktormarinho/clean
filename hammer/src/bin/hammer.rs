use clap::Parser;
use walkdir::WalkDir;

use hammer::fs_checks::{is_hidden, is_ignored};
use hammer::npm_process::NpmProcessContext;
use hammer::tasks;

#[derive(Parser, Debug)]
#[command(
    author = "viktormarinho", 
    version = "0.0.1", 
    about = "Hammer is a no-config cli tool for running concurrent tasks with monorepo support", 
    long_about = None)]
struct Args {
    command: String,

    #[arg(short, long)]
    filter: Option<String>,
}

#[tokio::main]
async fn main() {
    let args  = Args::parse();

    WalkDir::new(".")
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
    .map(|dir_entry| {
        NpmProcessContext::new(dir_entry, args.command.clone())
    })
    .filter(|ctx| {
        if let Some(filter) = &args.filter {
            return ctx.name == filter.to_owned()
        }
        true
    })
    .filter(|ctx| {
        ctx.contains_script()
    })
    .for_each(|process_context| {
        tasks::start_npm_process(process_context);
    })
}