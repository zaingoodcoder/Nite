mod commands;
mod improvement;
use commands::new::create_project;
use improvement::banner::print_banner;
use improvement::intro::starter_content;


use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "nite")]
#[command(version)]
#[command(
    name = "nite",
    version,
    about = "A CLI to quickly create projects",
    after_help = "\
Examples:
  nite new blog
  nite new api
  nite --help

GitHub:
  https://github.com/zaingoodcoder/Nite.git
"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    New {
        project_name: String,
    },
    Init,
}

fn main() {
    let args = Cli::parse();


    match args.command {
        Some(Commands::New { project_name } )=> {
            match create_project(&project_name){
                Ok(_) => {println!("project created")},
                Err(err) => {println!("{err}")}
            }
        },
        Some(Commands::Init) => { println!("init") },

        None => {
            print_banner();
            starter_content();
        }
    }
}