use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: &'a Vec<u8>,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            ip: &chunk.code,
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        let mut code_iterator = self.ip.iter();
        loop {
            let instruction = code_iterator.next();
            if instruction.is_none() {
                return InterpretResult::RuntimeError;
            }

            if let Ok(opcode) = OpCode::try_from(*instruction.unwrap()) {
                match opcode {
                    OpCode::OpReturn => return InterpretResult::Ok,
                    OpCode::OpConstant => {
                        let next_byte = code_iterator.next();
                        if next_byte.is_none() {
                            return InterpretResult::RuntimeError;
                        }

                        // This is not to be used in production. `next_byte` implies that there
                        // would be maximum 256 constants, which should not be the case.
                        // Multi-byte operations needed to be introduced to handle that
                        let constant: Value = self.chunk.constants[*next_byte.unwrap() as usize];
                        println!("{constant}");
                    }
                }
            }
        }
    }
}
