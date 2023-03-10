use std::fs;

use hammer_cli::errors::BeautifulErrors;

use crate::config::get_config;

#[derive(Debug, Clone, clap::Parser)]
pub struct CopyEnv;

impl CopyEnv {
    pub fn run(self) {
        let config = get_config();
        config.env_dirs.iter().for_each(|dir| {
            fs::copy(".env.dev", format!("{}.env", dir)).expect_or_err(
                &format!("Could not copy .env.dev contents to {}", dir)
            );
        });
    }
}