use clap::Parser;
use einer::commands::Commands;

#[derive(Parser, Debug, Clone)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

// https://3000-etseiner-einer-4p8pzjtskbo.ws-us89b.gitpod.io

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::ReplaceEnv(args) => einer::replace_env::run(args.to_owned())
    }
}