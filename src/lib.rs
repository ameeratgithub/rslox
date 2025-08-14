use std::process;

use crate::{
    chunk::Chunk,
    compiler::Compiler,
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

// Helper function which just logs if any errors are returned
fn execute(code: &str) {
    if let Err(e) = interpret(code) {
        match e {
            VMError::CompileError(e) => {
                eprintln!("Compiler Error: {}", e);
                process::exit(65);
            }
            VMError::RuntimeError(e) => {
                eprintln!("Runtime Error: {e}");
                process::exit(70);
            }
        }
    }
}

// A separate function which returns errors. Can be helpfull when writing tests
// to test against certain types of errors
pub fn interpret(code: &str) -> Result<(), VMError> {
    let mut chunk = Chunk::new();

    // It takes source code string and chunk variable. Updates the chunk variable
    // if compilation is successful
    let mut compiler = Compiler::new(code, &mut chunk);
    // Start compiling the code, if it returns error, just propagate the error.
    // If successful, updates the `self.chunk` field.
    compiler.compile().map_err(|e| VMError::CompileError(e))?;

    let mut vm: VM<'_> = VM::new(&chunk);
    vm.interpret()?;
    // Reset VM to its initial state. Frees garbage collection and resets stack.
    vm.reset_vm();

    Ok(())
}
