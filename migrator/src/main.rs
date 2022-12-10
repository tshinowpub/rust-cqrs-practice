use std::env;
use std::process::exit;

mod commands;
mod executor;
mod extractor;
pub mod command;

use crate::executor::Executor;
use crate::extractor::argument_extractor::ArgumentExtractor;
use crate::extractor::option_extractor::OptionExtractor;

fn main() {
    let execute_path: String;
    match env::current_exe() {
        Ok(exe_path) =>
            execute_path = exe_path.display().to_string(),
        Err(e) => {
            println!("Failed to get current execute path: {e}.");

            exit(1);
        },
    };

    let argument_extractor = ArgumentExtractor::new();
    let option_extractor = OptionExtractor::new();

    let arguments = argument_extractor.extract(env::args().collect(), &execute_path);
    let options = option_extractor.extract(env::args().collect(), &execute_path);

    match arguments.split_first() {
        Some((command, args)) if !args.is_empty() => {
            Executor::execute(command, &args.to_vec(), &options);
        },
        Some((command, _)) => {
            Executor::execute(command, &Vec::new(), &options);
        },
        _ => {
            println!("Use --help.");

            exit(1);
        }
    }

    exit(0);
}
