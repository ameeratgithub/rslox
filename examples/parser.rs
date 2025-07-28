use clap::Parser;
use rlox::Cli;

fn main() {
    let cli = Cli::parse();

    if let Some(file_path) = cli.file {
        println!("Should print file: {file_path}");
    } else {
        println!("Should start repl");
    }
}
