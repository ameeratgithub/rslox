use std::fmt::Arguments;
use std::fmt::Write as _;

use crate::{compiler::errors::CompilerError, vm::VM};

#[derive(Debug)]
/// Errors related to virtual machine
pub enum VMError {
    CompileError(CompilerError),
    RuntimeError(String),
}

/// This trait implementation makes it easier to customize error output, to look nicer.
impl std::fmt::Display for VMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CompileError(e) => {
                write!(f, "Compiler Error: {e}")
            }
            Self::RuntimeError(e) => {
                write!(f, "{e}")
            }
        }
    }
}

impl VM {
    /// This is important because we want to display errors nicely.
    /// It gets dynamic arguments, and constructs proper error
    pub(crate) fn construct_runtime_error(&mut self, arguments: Arguments) -> VMError {
        let mut message = format!("{arguments}\n");
        for frame in self.frames.iter().rev() {
            let function = &frame.function.as_function_ref();
            let instruction = frame.ip_offset - 1;
            let _ = write!(message, "[line {}] in ", function.chunk.lines[instruction]);

            if let Some(name) = function.name.as_ref() {
                let _ = writeln!(message, "{name}()");
            } else {
                message.push_str("<script>\n");
            }
        }

        // Error occured, reset stack.
        self.reset_vm();

        // Return proper error
        VMError::RuntimeError(message)
    }
}
