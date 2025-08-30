use std::{fs, process};

use crate::{
    compiler::{CompilationContext, CompilerState, types::FunctionType},
    value::FunctionObject,
    vm::{VM, VMError},
};

pub mod chunk;
pub mod cli;
pub mod compiler;
pub mod constants;
#[cfg(feature = "debug_trace_execution")]
pub mod debug;
pub mod scanner;
pub mod value;
pub mod vm;

// Helper function which just logs if any errors are returned
fn execute(code: &str, vm: &mut VM) {
    if let Err(e) = interpret(code, vm) {
        vm.reset_vm();
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
pub fn interpret(code: &str, vm: &mut VM) -> Result<(), VMError> {
    let mut context = CompilationContext::new(code);

    let function_type = FunctionType::Script(Box::new(FunctionObject::new()));
    context.push(CompilerState::new(function_type));
    let top_function = context.compile().map_err(|e| VMError::CompileError(e))?;

    // Value on stack should be garbage collected
    let stack_value = top_function.clone();
    vm.push(stack_value);

    vm.call(top_function, 0)?;
    vm.interpret()
}

/// Executes code from a file
pub fn run_file(file_path: &str) {
    let mut vm = VM::new();
    // Reads file and returns Result. If result is Ok, execute the string obtained from file
    if let Ok(content) = fs::read_to_string(file_path) {
        execute(&content, &mut vm);
        vm.reset_vm();
    } else {
        eprintln!("Can't read code from file: {file_path}");
        process::exit(74);
    }
}