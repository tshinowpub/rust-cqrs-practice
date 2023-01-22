use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;
use std::process::exit;
use crate::command::migrate_type::MigrateType;

use crate::command::Output;
use crate::command::migrate::Migrate as MigrateCommand;
use crate::command::list::List as ListCommand;

mod command;
mod clients;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Migrate {
        command: MigrateType,

        #[arg(short, long, required = false)]
        path: Option<PathBuf>,
    },
    List {
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    dbg!("{:?}", &cli);

    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {}", name);
    }

    dbg!("{}", &cli.command);

    match &cli.command {
        Some(Commands::Migrate { command, path}) => {
            let migrate = MigrateCommand::new();

            let output = migrate.execute(MigrateType::Up, path).await;

            println!("{}", output.message());

            exit(*(output.exit_code()) as i32);
        },
        Some(Commands::List {}) => {
            let list = ListCommand::new();

            let output = list.execute().await;

            println!("{}", output.message());

            exit(*(output.exit_code()) as i32);
        },
        None => {
            if let Some(name) = cli.name.as_deref() {
                println!("Command {} was not found.", name);
            }

            exit(0)
        }
    }
}
