use std::process::Command;
use hammer_cli::errors::BeautifulErrors;
use crate::log;

#[derive(Debug, Clone, clap::Parser)]
pub struct InitRepo;

impl InitRepo {
    pub fn run(self) {
        log::print("Instalando dependências e rodando scripts iniciais...");
        let mut repo_process = Command::new("pnpm")
            .current_dir(".")
            .arg("init-repo")
            .spawn()
            .expect_or_err("Não foi possível spawnar o comando pnpm init-repo");

        match repo_process.wait() {
            Ok(_) => log::print("Dependências instaladas."),
            Err(err) => {
                log::print_err("Erro durante a execução do redis.");
                log::print_err(&format!("Error: {err}"));
            }
        }
    }
}