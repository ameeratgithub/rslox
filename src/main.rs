use crate::{
    chunk::{Chunk, OpCode},
    vm::{InterpretResult, VM},
};

#[cfg(feature = "debug_trace_execution")]
use crate::debug::Debug;

// pub mod common;
pub mod chunk;
#[cfg(feature = "debug_trace_execution")]
pub mod debug;
pub mod value;
pub mod vm;
fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    // Line number is arbitrary at this point
    chunk.write_chunk(OpCode::OpConstant as u8, 123);
    
    // This implementation is deeply flawed, and assumes that constant value would
    // not be greater than 255. It's the conversion in Rust, but should be handled
    // carefully.
    chunk.write_chunk(constant as u8, 123);

    chunk.write_chunk(OpCode::OpReturn as u8, 123);

    #[cfg(feature = "debug_trace_execution")]
    Debug::dissassemble_chunk(&chunk, "Test Chunk");

    let mut vm = VM::new(&chunk);
    let result = vm.interpret();

    match result {
        InterpretResult::Ok => println!("Code successfully interpreted "),
        InterpretResult::CompileError => eprintln!("Error: Can't compile the code"),
        InterpretResult::RuntimeError => eprintln!("Error: A runtime error occurred"),
    }
}
