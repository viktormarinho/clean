use hammer_cli::errors::BeautifulErrors;
use std::fs;

use crate::copy_env::CopyEnv;

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

    pub fn run_setup(self) {
        let orig_env = fs::read_to_string(".env.dev").expect_or_err("Não consegui achar o arquivo .env.dev. Você está executando este comando da root do projeto?");

        GitpodUrl{gitpod_url: self.gitpod_url}.run_replace();
        CopyEnv{}.run();

        fs::write(".env.dev", orig_env)
            .expect_or_err("Não foi possível voltar o arquivo .env.dev ao seu estado original");
    }
}
