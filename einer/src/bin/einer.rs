use clap::Parser;
use einer::commands::Commands;

#[derive(Parser, Debug, Clone)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::ReplaceEnv(args) => args.to_owned().run_replace(),
        Commands::CopyEnv(args) => args.to_owned().run(),
        Commands::SetupEnv(args) => args.to_owned().run_setup(),
        Commands::RunRedis(args) => args.to_owned().run(),
        Commands::InitRepo(args) => args.to_owned().run(),
        Commands::StartGitpod(args) => args.to_owned().start(),
    }
}