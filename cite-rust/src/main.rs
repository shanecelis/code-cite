use async_trait::async_trait;
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
use handlebars::{handlebars_helper, Handlebars, JsonRender};
use std::fmt::Display;

pub mod twitter;
pub type GenericError = Box<dyn Error + Send + Sync>;

fn my_inline_link() -> i32 {
    static mut i : i32 = 0;
    unsafe {
        i += 1;
        i
    }
}

fn my_ref_link() -> i32 {
    static mut i : i32 = 0;
    unsafe {
        i += 1;
        i
    }
}

handlebars_helper!(inline_link: | | //{name:Option<String>}|
                   Some(my_inline_link().to_string())
);

handlebars_helper!(ref_link: | | //{name:Option<String>}|
                   Some(my_ref_link().to_string())
);


#[async_trait]
pub trait Plugin {
    async fn fill(&self) -> Result<HashMap<String,String>, GenericError>;
}

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
    note: Option<String>,
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
    let mut data : HashMap<String, String> = HashMap::new();
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

    fn insert_if<T : Display>(data:&mut HashMap<String, String>, name: &str, value:&Option<T>) {
        if let Some(x) = value {
            data.insert(name.to_string(), x.to_string());
        }
    }
    insert_if(&mut data, "author_name", &cli.author_name);
    insert_if(&mut data, "year_created", &cli.year_created);
    insert_if(&mut data, "author_link", &cli.author_link);
    insert_if(&mut data, "modifier_name", &cli.modifier_name);
    insert_if(&mut data, "year_modified", &cli.year_modified);
    insert_if(&mut data, "modifier_link", &cli.modifier_link);
    insert_if(&mut data, "code_url", &cli.code_url);
    insert_if(&mut data, "code_url_modified", &cli.code_url_modified);
    insert_if(&mut data, "note", &cli.note);
    if let Some(license) = cli.license {
        if let Some(item) = &config.licenses.get(&license) {
            data.insert("license_name".to_string(), item.name.to_string());
            data.insert("license_link".to_string(), item.link.to_string());
        } else {
            data.insert("license_name".to_string(), license.to_string());
        }
    }
    insert_if(&mut data, "code_cite_link", &if cli.no_link { None } else { Some("http://cite github.com") });
    // if let Some(author) = cli.author_name {
    //     data.insert("author_name".to_string(), author);
    // }

    if let Some(year_created) = cli.year_created {
        data.insert("year_created".to_string(), year_created);
    }
    // data.insert("author_name".to_string(), author);
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);

    handlebars.register_helper("inline_link", Box::new(inline_link));
    handlebars.register_helper("ref_link", Box::new(ref_link));
    let mut links = vec!["hi", "bye"];
    let closure = || links.connect(",");

handlebars_helper!(ref_links: | | {
                   Some(closure())
}
);
    handlebars.register_helper("ref_links", Box::new(ref_links));

    handlebars.register_templates_directory(".mo", home + "templates").unwrap();
    // println!("Hello, world!");
    println!("{}", handlebars.render("stackoverflow", &data).unwrap());
}
