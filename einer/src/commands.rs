use clap::Subcommand;

use crate::replace_env::ReplaceEnv;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    ReplaceEnv(ReplaceEnv),
}