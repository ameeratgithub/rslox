use rslox::{
    chunk::{Chunk, OpCode},
    vm::{VM, VMError},
};

// #[cfg(feature = "debug_trace_execution")]
// use rslox::debug::Debug;

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
    let mut vm = VM::new(&mut chunk);
    // Because we've directly written bytecode, there is no source code to be compiled
    // So directly call `vm.run()` instead of `vm.interpret()` 
    if let Err(e) = vm.run() {
        match e {
            VMError::CompileError(e) => eprintln!("Compilation Error:{e}"),
            VMError::RuntimeError => eprintln!("Error: A runtime error occurred"),
        }
    }
}
