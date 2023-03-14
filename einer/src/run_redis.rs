use std::process::Command;
use hammer_cli::errors::BeautifulErrors;



#[derive(Debug, Clone, clap::Parser)]
pub struct RunRedis;

impl RunRedis {
    pub fn run(self) {
        Command::new("pnpm")
            .current_dir(".")
            .arg("redis-run")
            .spawn()
            .expect_or_err("Não foi possível spawnar o comando pnpm redis-run");
    }
}