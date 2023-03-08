use hammer_cli::errors::BeautifulErrors;
use std::fs;

#[derive(Debug, Clone, clap::Parser)]
pub struct ReplaceEnv {
    gitpod_url: String
}

pub fn run(args: ReplaceEnv) {

    let port = {
        let sliced: Vec<&str> = args.gitpod_url.split("//").collect();
        sliced
            .get(1)
            .expect_or_err("Url do gitpod não está formatada corretamente")
            .split("-")
            .collect::<Vec<&str>>()
            .get(0)
            .expect_or_err("Url do gitpod não está formatada corretamente")
            .to_string()
    };

    let env_file = fs::read_to_string(".env.dev").expect_or_err("Não consegui achar o arquivo .env.dev. Você está executando este comando da root do projeto?");

    let mut fixed_env = String::new();

    for line in env_file.lines() {
        let line_port = {
            let sliced: Vec<&str> = line.split(":").collect();

            match sliced.get(2) {
                Some(val) => val,
                None => ""
            }
        };

        let line = line.replace(&format!("=http://localhost:{}", line_port), &format!("={}", &args.gitpod_url.replace(&port, line_port)));

        fixed_env.push_str(&line);
        fixed_env.push_str("\n");
    }

    fs::write(".env.dev", fixed_env).expect_or_err("Não foi possivel sobrescrever o arquivo .env.dev");
}