pub mod constants;
#[cfg(feature = "debug_trace_execution")]
use crate::debug::Debug;

use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
    vm::constants::STACK_MAX,
};

pub enum VMError {
    CompileError,
    RuntimeError,
}

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip_offset: usize,
    stack: [Value; STACK_MAX as usize],
    // // Should it really be a pointer?
    stack_top: usize,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            ip_offset: 0,
            stack: [0.0; STACK_MAX as usize],
            stack_top: 0,
        }
    }

    pub fn interpret(&mut self) -> Result<(), VMError> {
        self.run()
    }

    pub fn reset_stack(&mut self) {
        self.stack = [0.0; STACK_MAX as usize];
        self.stack_top = 0;
    }

    pub fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    pub fn pop(&mut self) -> Option<Value> {
        if self.stack_top == 0 {
            return None;
        }

        self.stack_top -= 1;
        Some(self.stack[self.stack_top])
    }

    fn read_constant(&mut self) -> Value {
        let constant_position = self.chunk.code[self.ip_offset];
        // This is not to be used in production. `next_byte` implies that there
        // would be maximum 256 constants, which should not be the case.
        // Multi-byte operations needed to be introduced to handle that
        let constant: Value = self.chunk.constants[constant_position as usize];
        self.ip_offset += 1;
        constant
    }

    fn binary_op(&mut self, opcode: OpCode) -> Result<(), VMError> {
        let right_operand = self.pop().ok_or_else(|| VMError::CompileError)?;
        let left_operand = self.pop().ok_or_else(|| VMError::CompileError)?;

        let result = match opcode {
            OpCode::OpAdd => left_operand + right_operand,
            OpCode::OpSubtract => left_operand - right_operand,
            OpCode::OpMultiply => left_operand * right_operand,
            OpCode::OpDivide => left_operand / right_operand,
            // This arm should never be matched.
            _ => unreachable!(),
        };

        self.push(result);
        Ok(())
    }

    fn run(&mut self) -> Result<(), VMError> {
        loop {
            #[cfg(feature = "debug_trace_execution")]
            {
                print!("          ");
                for slot in 0..self.stack_top {
                    print!("[ ");
                    print!("{}", self.stack[slot]);
                    print!(" ]");
                }
                println!("");
                Debug::dissassemble_instruction(&self.chunk, self.ip_offset);
            }

            let instruction_byte = self.chunk.code[self.ip_offset];
            self.ip_offset += 1;

            if let Ok(opcode) = OpCode::try_from(instruction_byte) {
                match opcode {
                    OpCode::OpReturn => {
                        println!("{}", self.pop().unwrap());
                        return Ok(());
                    }
                    OpCode::OpConstant => {
                        let constant = self.read_constant();
                        self.push(constant);
                    }
                    OpCode::OpNegate => {
                        let negated_value = -self.pop().ok_or_else(|| VMError::RuntimeError)?;
                        self.push(negated_value);
                    }
                    OpCode::OpAdd | OpCode::OpSubtract | OpCode::OpMultiply | OpCode::OpDivide => {
                        self.binary_op(opcode)?
                    }
                }
            }
        }
    }
}
