use std::{
    fs,
    io::{self, Write},
    process,
};

use clap::Parser;

use crate::execute;

#[derive(Parser, Debug)]
#[command(author,version, about, long_about=None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<String>,
}

pub fn repl() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut line = String::new();

    loop {
        print!("> ");

        if let Err(e) = stdout.flush() {
            eprintln!("Error flushing stdout: {}", e);
            break;
        }

        match stdin.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!();
                    break;
                }

                let source = line.trim_end();
                if source.is_empty() {
                    line.clear();
                    continue;
                }

                execute(source);
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
        line.clear();
    }
}

pub fn run_file(file_path: &str) {
    if let Ok(content) = fs::read_to_string(file_path) {
        execute(&content);
    } else {
        eprintln!("Can't read code from file: {file_path}");
        process::exit(74);
    }
}
