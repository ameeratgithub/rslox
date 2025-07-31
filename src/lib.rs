use std::process;

use crate::{
    chunk::Chunk,
    vm::{VM, VMError},
};

pub mod chunk;
pub mod cli;
pub mod compiler;
#[cfg(feature = "debug_trace_execution")]
pub mod debug;
pub mod scanner;
pub mod value;
pub mod vm;

fn execute(code: &str) {
    if let Err(e) = interpret(code) {
        match e {
            VMError::CompileError(e) => {
                eprintln!("{}", e);
                process::exit(65);
            }
            VMError::RuntimeError => {
                process::exit(70);
            }
        }
    }
}

pub fn interpret(code: &str) -> Result<(), VMError> {
    let mut chunk = Chunk::new();
    let mut vm = VM::new(&mut chunk);
    vm.interpret(code)?;
    Ok(())
}
