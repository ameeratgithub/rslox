/// Debug module to print instructions in debug_trace_execution mode
use crate::chunk::{Chunk, OpCode};

/// Struct doesn't have any properties
pub struct Debug;

impl Debug {
    /// Receives a chunk, and a name for that chunk, and print instructions
    pub fn dissassemble_chunk(chunk: &Chunk, name: &str) {
        println!("== {name} ==");

        // Starting from 0 offset
        let mut offset = 0;

        // if offset is less than byte code length, print instruction and update the offset
        while offset < chunk.code.len() {
            offset = Debug::dissassemble_instruction(chunk, offset);
        }
    }

    // Print the current instruction and returns new offset
    pub fn dissassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
        print!("{:04} ", offset);

        // If offset is greater than 0, i.e. at least one byte has been processed before
        // and previous byte and this byte is on the same line, just print a '|'
        if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
            print!("   | ");
        } else {
            // print line number
            print!("{: >4} ", chunk.lines[offset]);
        }

        // First byte of code is consumed here.
        let instruction = chunk.code[offset];
        // Convert u8 to OpCode
        if let Ok(opcode) = OpCode::try_from(instruction) {
            match opcode {
                OpCode::OpReturn => Debug::simple_instruction("OpReturn", offset),
                OpCode::OpConstant => Debug::constant_instruction("OpConstant", chunk, offset),
                OpCode::OpNegate => Debug::simple_instruction("OpNegate", offset),
                OpCode::OpAdd => Debug::simple_instruction("OpAdd", offset),
                OpCode::OpSubtract => Debug::simple_instruction("OpSubtract", offset),
                OpCode::OpMultiply => Debug::simple_instruction("OpMultiply", offset),
                OpCode::OpDivide => Debug::simple_instruction("OpDivide", offset),
                OpCode::OpNil => Debug::simple_instruction("OpNil", offset),
                OpCode::OpTrue => Debug::simple_instruction("OpTrue", offset),
                OpCode::OpFalse => Debug::simple_instruction("OpFalse", offset),
                OpCode::OpNot => Debug::simple_instruction("OpNot", offset),
                OpCode::OpEqual => Debug::simple_instruction("OpEqual", offset),
                OpCode::OpGreater => Debug::simple_instruction("OpGreater", offset),
                OpCode::OpLess => Debug::simple_instruction("OpLess", offset),
                OpCode::OpPrint => Debug::simple_instruction("OpPrint", offset),
                OpCode::OpPop => Debug::simple_instruction("OpPop", offset),
                OpCode::OpDefineGlobal => {
                    Debug::constant_instruction("OpDefineGlobal", chunk, offset)
                }
                OpCode::OpGetGlobal => Debug::constant_instruction("OpGetGlobal", chunk, offset),
                OpCode::OpSetGlobal => Debug::constant_instruction("OpSetGlobal", chunk, offset),
                OpCode::OpGetLocal => Debug::byte_instruction("OpGetLocal", chunk, offset),
                OpCode::OpSetLocal => Debug::byte_instruction("OpSetLocal", chunk, offset),
                OpCode::OpJump => Debug::jump_instruction("OpJump", 1, chunk, offset),
                OpCode::OpJumpIfFalse => Debug::jump_instruction("OpJumpIfFalse", 1, chunk, offset),
                OpCode::OpLoop => Debug::jump_instruction("OpLoop", -1, chunk, offset),
            }
        } else {
            // Print invalid instruction error
            eprintln!("Can't fetch relevant OpCode. Invalid instruction: {instruction}");
            // Consume the construction and return new offset
            offset + 1
        }
    }

    /// Print the constant instruction and returns new offset
    fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
        // First byte has been consumed, which is OpCode. Second byte will be the offset of the constant
        let constant_index = chunk.code[offset + 1];
        // Print the name of the instruction, and offset of the constant
        print!("{: <16} {: >4} '", name, constant_index);
        // Print the actuall constant value
        println!("{}'", chunk.constants[constant_index as usize]);
        // Constant instruction/opcode has 2 bytes, consumed both bytes so new offset would be offset + 2
        offset + 2
    }

    // Prints simple instruction and returns new offset
    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{name}");
        // Since simple instruction is one byte, new offset would be offset + 1
        offset + 1
    }

    fn byte_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
        let slot = chunk.code[offset + 1];
        print!("{: <16} {: >4} '", name, slot);
        offset + 2
    }

    fn jump_instruction(name: &str, sign: isize, chunk: &Chunk, offset: usize) -> usize {
        let jump = u16::from_be_bytes([chunk.code[offset + 1], chunk.code[offset + 2]]);
        println!(
            "{: <16} {: >4} -> {}",
            name,
            offset,
            ((offset + 3) as isize) + sign * (jump as isize)
        );

        offset + 3
    }
}
