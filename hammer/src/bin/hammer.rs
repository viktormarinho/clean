use clap::Parser;
use walkdir::WalkDir;

use hammer_cli::fs_checks::{is_hidden, is_ignored};
use hammer_cli::npm_process::NpmProcessContext;
use hammer_cli::tasks;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    command: String,

    #[arg(short, long)]
    filter: Option<String>,

    #[arg(short, long, default_value_t = false)]
    no_prefix: bool,
}

#[tokio::main]
async fn main() {
    let args  = Args::parse();
    dotenv::dotenv().ok();

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
    .filter_map(|ctx| {
        ctx.validate_script(args.no_prefix)
    })
    .for_each(|process_context| {
        tasks::start_npm_process(process_context);
    })
}