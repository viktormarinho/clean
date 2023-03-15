use std::process::Command;
use hammer_cli::errors::BeautifulErrors;
use crate::log;

#[derive(Debug, Clone, clap::Parser)]
pub struct RunRedis;

impl RunRedis {
    pub fn run(self) {
        log::print("Instalando e iniciando o redis...");
        let mut redis_process = Command::new("pnpm")
            .current_dir(".")
            .arg("redis-run")
            .spawn()
            .expect_or_err("Não foi possível spawnar o comando pnpm redis-run");

        match redis_process.wait() {
            Ok(_) => log::print("Redis rodando."),
            Err(err) => {
                log::print_err("Erro durante a execução do redis.");
                log::print_err(&format!("Error: {err}"));
            }
        }
    }
}