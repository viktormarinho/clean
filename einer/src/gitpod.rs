use hammer_cli::errors::BeautifulErrors;
use std::{fs, process::Command};

use crate::{copy_env::CopyEnv, run_redis::RunRedis, init_repo::InitRepo, log, config};

#[derive(Debug, Clone, clap::Parser)]
pub struct GitpodUrl {
    // Your current gitpod url
    gitpod_url: String,
}

// Gitpod url example
// https://viktormarinho-clean-wn1h09roiej.ws-us89b.gitpod.io/

impl GitpodUrl {
    pub fn run_replace(self) {
        let replaceable_url = {
            let sliced: Vec<&str> = self.gitpod_url.split("//").collect();
            format!(
                "{}//XXXX-{}", 
                sliced.get(0).expect_or_err("Url do gitpod não está formatada corretamente"), 
                sliced.get(1).expect_or_err("Url do gitpod não está formatada corretamente")
            )
        };

        let replaceable_url = {
            if replaceable_url.chars().last().unwrap() == '/' {
                replaceable_url[0..replaceable_url.len() - 1].to_string()
            } else {
                replaceable_url
            }
        };

        let env_file = fs::read_to_string(".env.dev").expect_or_err("Não consegui achar o arquivo .env.dev. Você está executando este comando da root do projeto?");

        let mut fixed_env = String::new();

        for line in env_file.lines() {
            let line_port = {
                let sliced: Vec<&str> = line.split(":").collect();

                match sliced.get(2) {
                    Some(val) => val,
                    None => "",
                }
            };

            let line = line.replace(
                &format!("=http://localhost:{}", line_port),
                &format!("={}", &replaceable_url.replace("XXXX", line_port)),
            );

            fixed_env.push_str(&line);
            fixed_env.push_str("\n");
        }

        fs::write(".env.dev", fixed_env)
            .expect_or_err("Não foi possivel sobrescrever o arquivo .env.dev");
    }

    pub fn run_setup(&self) {
        let orig_env = fs::read_to_string(".env.dev").expect_or_err("Não consegui achar o arquivo .env.dev. Você está executando este comando da root do projeto?");

        GitpodUrl{gitpod_url: self.gitpod_url.to_owned()}.run_replace();
        CopyEnv{}.run();

        fs::write(".env.dev", orig_env)
            .expect_or_err("Não foi possível voltar o arquivo .env.dev ao seu estado original");
    }

    pub fn expose_ports(&self) {
        let cfg = config::get_config();

        cfg.open_ports.iter().for_each(|port| {
            // Command::new("gp")
            //     .current_dir(".")
            //     .arg("ports")
            //     .arg("expose")
            //     .arg(port)
            //     .spawn()
            //     .expect_or_err(&format!("Não foi possível spawnar o comando 'gp ports expose' na porta {port}"))
            //     .wait()
            //     .expect_or_err(&format!("Erro ao expor a porta {port}"));

            Command::new("gp")
                .current_dir(".")
                .arg("ports")
                .arg("visibility")
                .arg(&format!("{port}:public"))
                .spawn()
                .expect_or_err(&format!("Não foi possível spawnar o comando 'gp ports visibility' na porta {port}"))
                .wait()
                .expect_or_err(&format!("Erro ao deixar a porta {port} pública"));
        });
    }

    pub fn start(self) {
        log::print("Iniciando configuração do gitpod...");

        log::print("Configurando variáveis de ambiente...");
        self.run_setup();
        
        RunRedis.run();
        InitRepo.run();

        self.expose_ports();

        log::print("Tudo pronto! Basta começar a desenvolver usando 'pnpm dev'");
    }
}
