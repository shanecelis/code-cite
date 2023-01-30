use clap::{arg, command, Parser};
use std::collections::HashMap;
use serde::Deserialize;
use serde_json::json;
use std::process::exit;
use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::BufReader;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short = 'u', long)]
    code_url: Option<String>,
    #[arg(short = 'U', long)]
    code_url_modified: Option<String>,
    #[arg(short = 'x', long)]
    author_name: Option<String>,
    #[arg(short = 'y', long)]
    year_created: Option<String>,
    #[arg(short = 'z', long)]
    author_link: Option<String>,
    #[arg(short = 'X', long)]
    modifier_name: Option<String>,
    #[arg(short = 'Y', long)]
    year_modified: Option<String>,
    #[arg(short = 'Z', long)]
    modifier_link: Option<String>,
    #[arg(short = 'l', long)]
    license: Option<String>,
    #[arg(short = 'N', long)]
    notes: Option<String>,
    #[arg(short = 'p', long)]
    prefix: Option<String>,
    #[arg(short = 'c', long)]
    config: Option<String>,
    #[arg(short = 't', long)]
    template: Option<String>,
    #[arg(short = 'v')]
    verbose: bool,
    #[arg(short = 'm')]
    modified: bool,
    #[arg(short = 'M')]
    not_modified: bool,
    #[arg(short = 'C')]
    no_link: bool,
    #[arg(short = 'T')]
    list_templates: bool,
    #[arg(short = 'L')]
    list_licenses: bool,
}

#[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
struct Item {
    name: String,
    link: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    version: String,
    contributors: HashMap<String, Item>,
    licenses: HashMap<String, Item>,
}

fn main() {
    let cli = Cli::parse();
    let home = "/Users/shanecelis/Projects/code-cite/".to_owned();
    let file = File::open(home.clone() + "cite-config.json")
    .expect("file should open read only");
    let config: Config = serde_json::from_reader(BufReader::new(file)).expect("JSON was not well-formatted");
    if cli.list_licenses {
        for license in config.licenses.keys() {
            println!("{}", license);
        }
        exit(0);
    } else if cli.list_templates {
        let paths = fs::read_dir(home + "templates").unwrap();

        for path in paths {
            let p = path.unwrap().path();
            let basename = p.file_stem().unwrap().to_str().unwrap();
            println!("{}", basename);
        }
        exit(0);
    }

    println!("Hello, world!");
}
