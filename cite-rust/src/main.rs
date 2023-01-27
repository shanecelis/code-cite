use clap::{arg, command, Parser};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short = 'u', long)]
    code_url: Option<String>,
    #[arg(short = 'x', long)]
    author_name: Option<String>
}

fn main() {
    let cli = Cli::parse();

    println!("Hello, world!");
}
