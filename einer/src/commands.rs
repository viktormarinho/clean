use clap::Subcommand;

use crate::gitpod_env::GitpodUrl;
use crate::copy_env::CopyEnv;
use crate::run_redis::RunRedis;
use crate::init_repo::InitRepo;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    // Replaces "localhost" environment variables with a given gitpod_url 
    ReplaceEnv(GitpodUrl),
    // Copies .env.dev contents to a .env file in all the env_dirs at the einer.toml config file
    CopyEnv(CopyEnv),
    // Runs replace-env, copy-env and then returns the root .env.dev to the original state
    SetupEnv(GitpodUrl),
    // Runs einer package.json's "run-redis" script 
    RunRedis(RunRedis),
    // Runs einer package.json's "init-repo" script
    InitRepo(InitRepo),
}