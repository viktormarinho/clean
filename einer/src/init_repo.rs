use std::process::Command;
use hammer_cli::errors::BeautifulErrors;

#[derive(Debug, Clone, clap::Parser)]
pub struct InitRepo;

impl InitRepo {
    pub fn run(self) {
        Command::new("pnpm")
            .current_dir(".")
            .arg("init-repo")
            .spawn()
            .expect_or_err("Não foi possível spawnar o comando pnpm init-repo");
    }
}