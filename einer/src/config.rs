use hammer_cli::errors::BeautifulErrors;
use serde::Deserialize;

use crate::log;


#[derive(Debug, Deserialize)]
pub struct EinerConfig {
    pub env_dirs: Vec<String>,
    pub open_ports: Vec<String>
}

pub fn get_config() -> EinerConfig {
    let file_str = std::fs::read_to_string("./einer.toml").expect_or_err("Não foi possível achar o arquivo einer.toml na raiz do projeto");

    let config: EinerConfig = toml::from_str(&file_str).expect_or_err("O arquivo einer.toml possui sintaxe incorreta");

    log::print("Campos esperados no einer.toml atualmente: env_dirs, open_ports");

    return config;
}