use clap::Parser;
use rslox::cli::{Cli, repl, run_file};

fn main() {
    let cli = Cli::parse();

    if let Some(file_path) = cli.file {
        run_file(&file_path);
    } else {
        repl();
    }
}