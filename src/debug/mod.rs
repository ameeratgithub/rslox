use crate::chunk::{Chunk, OpCode};

pub struct Debug;

impl Debug {
    pub fn dissassemble_chunk(chunk: &Chunk, name: &str) {
        println!("== {name} ==");

        let mut offset = 0;
        while offset < chunk.code.len() {
            offset = Debug::dissassemble_instruction(chunk, offset);
        }
    }

    pub fn dissassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
        print!("{:04} ", offset);

        if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{: >4} ", chunk.lines[offset]);
        }

        // First byte of code is consumed here.
        let instruction = chunk.code[offset];
        if let Ok(opcode) = OpCode::try_from(instruction) {
            match opcode {
                OpCode::OpReturn => Debug::simple_instruction("OpReturn", offset),
                OpCode::OpConstant => Debug::constant_instruction("OpConstant", chunk, offset),
                OpCode::OpNegate => Debug::simple_instruction("OpNegate", offset),
                OpCode::OpAdd => Debug::simple_instruction("OpAdd", offset),
                OpCode::OpSubtract => Debug::simple_instruction("OpSubtract", offset),
                OpCode::OpMultiply => Debug::simple_instruction("OpMultiply", offset),
                OpCode::OpDivide => Debug::simple_instruction("OpDivide", offset),
            }
        } else {
            eprintln!("Can't fetch relevant OpCode. Invalid instruction: {instruction}");
            offset + 1
        }
    }

    fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
        // Will start from second byte of instruction
        let constant_index = chunk.code[offset + 1];
        print!("{: <16} {: >4} '", name, constant_index);
        print!("{}", chunk.constants[constant_index as usize]);
        println!("'");
        offset + 2
    }

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }
}
