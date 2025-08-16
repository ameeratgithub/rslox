/// This module handles CLI arguments and takes actions. Simplified using `clap` crate
use std::{
    fs,
    io::{self, Write},
    process,
};

use crate::{execute, vm::VM};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author,version, about, long_about=None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<String>,
}

/// Starts a repl and execute code
pub fn repl() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut line = String::new();

    // let mut chunk = Chunk::new();
    let mut vm = VM::new();

    loop {
        print!("> ");

        // Display `>` on the screen.
        if let Err(e) = stdout.flush() {
            eprintln!("Error flushing stdout: {}", e);
            break;
        }

        // Read complete line. If it's successful, update the line variable
        match stdin.read_line(&mut line) {
            // reading line is successful
            Ok(bytes_read) => {
                // total bytes read shouldn't be '0'. Exit if value is '0'
                if bytes_read == 0 {
                    println!();
                    break;
                }

                // remove all whitespaces from the end
                let source = line.trim_end();
                // if input is empty after removing spaces, there's no need to execute anything
                // continue to ask for new input
                if source.is_empty() {
                    line.clear();
                    continue;
                }

                // If user typed exit, just like many repls, quit the CLI.
                if source == "exit" {
                    break;
                }

                // just run the code and display errors if any
                execute(source, &mut vm);
            }
            // Display error if reading line from cli is unsuccessful
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
        // clear/empty the line for new input.
        line.clear();
    }
}

/// Executes code from a file
pub fn run_file(file_path: &str) {
    let mut vm = VM::new();

    // Reads file and returns Result. If result is Ok, execute the string obtained from file
    if let Ok(content) = fs::read_to_string(file_path) {
        execute(&content, &mut vm);
    } else {
        eprintln!("Can't read code from file: {file_path}");
        process::exit(74);
    }
}
