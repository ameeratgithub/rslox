use crate::{
    chunk::{Chunk, OpCode},
    vm::{VM, VMError},
};

// #[cfg(feature = "debug_trace_execution")]
// use crate::debug::Debug;

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

    let constant = chunk.add_constant(3.4);
    chunk.write_chunk(OpCode::OpConstant as u8, 123);
    chunk.write_chunk(constant as u8, 123);

    chunk.write_chunk(OpCode::OpAdd as u8, 123);

    let constant = chunk.add_constant(5.6);
    chunk.write_chunk(OpCode::OpConstant as u8, 123);
    chunk.write_chunk(constant as u8, 123);

    chunk.write_chunk(OpCode::OpDivide as u8, 123);

    chunk.write_chunk(OpCode::OpNegate as u8, 123);
    chunk.write_chunk(OpCode::OpReturn as u8, 123);

    // #[cfg(feature = "debug_trace_execution")]
    // Debug::dissassemble_chunk(&chunk, "Test Chunk");
    let mut vm = VM::new(&chunk);
    if let Err(e) = vm.interpret() {
        match e {
            VMError::CompileError => eprintln!("Error: Can't compile the code"),
            VMError::RuntimeError => eprintln!("Error: A runtime error occurred"),
        }
    }
}
