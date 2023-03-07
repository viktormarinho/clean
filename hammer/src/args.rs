use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub script: String,

    #[arg(short, long)]
    pub filter: Option<String>,

    #[arg(short, long, default_value_t = false)]
    pub no_prefix: bool,

    #[arg(short, long)]
    pub env: Option<Vec<String>>,

    #[arg(short, long)]
    pub depth: Option<usize>
}