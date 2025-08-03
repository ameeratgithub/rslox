/// This module handles all the stuff a VM is supposed to do
/// It takes source code, compiles it, gets bytecode (stored in chunk) from compiler
/// and then execute that bytecode
pub mod constants;
use std::fmt::Arguments;

/// A custom `feature` to enable execution tracing.
/// When enabled, instructions are printed to console to see how bytecode is working
#[cfg(feature = "debug_trace_execution")]
use crate::debug::Debug;

use crate::{
    chunk::{Chunk, OpCode},
    compiler::{Compiler, CompilerError},
    value::Value,
    vm::constants::STACK_MAX,
};

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
                write!(f, "Compiler Error: {}", e)
            }
            Self::RuntimeError(e) => {
                write!(f, "Runtime Error: {e}")
            }
        }
    }
}

/// Data structure to handle a stack based virtual machine
pub struct VM<'a> {
    /// A mutable reference to the `Chunk`.
    chunk: &'a mut Chunk,
    /// Instruction pointer offset.
    ip_offset: usize,
    /// Stack to handle variables. Fixed stack size for simplicity, but has some limitations
    stack: [Value; STACK_MAX as usize],
    /// A pointer to check where we're on our stack. If value is 0, stack is empty.
    stack_top: usize,
}

impl<'a> VM<'a> {
    /// Returns a new instance of the VM
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            chunk,
            ip_offset: 0,
            stack: [Value::Nil; STACK_MAX as usize],
            stack_top: 0,
        }
    }

    /// Compiles source code, gets bytecode from compiler, and executes that bytecode
    pub fn interpret(&mut self, source: &'a str) -> Result<(), VMError> {
        // It takes source code string and chunk variable. Updates the chunk variable
        // if compilation is successful
        let mut compiler = Compiler::new(source, self.chunk);
        // Start compiling the code, if it returns error, just propagate the error.
        // If successful, updates the `self.chunk` field.
        compiler.compile().map_err(|e| VMError::CompileError(e))?;

        // Run the bytecode (`self.chunk`) received from compiler.
        self.run()
    }

    // Empties the stack and resets the top to '0'
    pub fn reset_stack(&mut self) {
        self.stack = [Value::Nil; STACK_MAX as usize];
        self.stack_top = 0;
    }

    // Push the value to stack, and increments the top
    pub fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    // Pop the value from stack, and decrements the top
    pub fn pop(&mut self) -> Option<Value> {
        // Decrement top before accessing the element because top is one step ahead
        self.stack_top = self.stack_top.checked_sub(1)?;

        // Returning the top value from the stak. No need to delete value,
        // just manage the stack pointer (stack_top)
        Some(self.stack[self.stack_top])
    }

    /// Reads constant from constant pool
    fn read_constant(&mut self) -> Value {
        // We don't directly store constants on bytecode. Bytecode has the
        // index/offset of constant. We get that index from bytecode.
        let constant_position = self.chunk.code[self.ip_offset];
        // Gets the value from constant pool.
        // This is not to be used in production. `constant_position` implies that there
        // would be maximum 256 constants, which should not be the case.
        // Multi-byte operations needed to be introduced to handle that
        let constant: Value = self.chunk.constants[constant_position as usize];
        // increment instruction pointer by 1, because we've consumed 1 byte
        self.ip_offset += 1;
        // return the value
        constant
    }

    // Performs the binary operation based on `opcode`.
    // `binary_op` should only be called when `opcode` supports binary operation.
    fn binary_op(&mut self, opcode: OpCode) -> Result<(), VMError> {
        // We're reading from left to right. So left operand got pushed first, then the right
        // operand got pushed. Let's say we're evaluating following expression
        // 2 - 1
        // 1. 2 got pushed -> [2]
        // 2. 1 got pushed -> [2,1]
        // 3. 1 is right operand, and will be popped first, because it's on top
        // 4. 2 is left operand, and it will be popped second.
        // 5. so the correct operation will be left_operand - right_operand
        let right_operand = self
            .pop()
            .ok_or_else(|| {
                let err = format_args!("Expected value on stack");
                self.construct_runtime_error(err)
            })
            .and_then(|val| {
                if val.is_number() {
                    Ok(val)
                } else {
                    let err = format_args!("Expected number as right operand");
                    Err(self.construct_runtime_error(err))
                }
            })?;

        let left_operand = self
            .pop()
            .ok_or_else(|| {
                let err = format_args!("Expected value on stack");
                self.construct_runtime_error(err)
            })
            .and_then(|val| {
                if val.is_number() {
                    Ok(val)
                } else {
                    let err = format_args!("Expected number as left operand");
                    Err(self.construct_runtime_error(err))
                }
            })?;

        // Match the opcode and perform the relevant operation
        let result = match opcode {
            OpCode::OpAdd => left_operand + right_operand,
            OpCode::OpSubtract => left_operand - right_operand,
            OpCode::OpMultiply => left_operand * right_operand,
            OpCode::OpDivide => left_operand / right_operand,
            OpCode::OpGreater => {
                let res = left_operand.to_number() > right_operand.to_number();
                Value::Bool(res)
            }
            OpCode::OpLess => {
                let res = left_operand.to_number() < right_operand.to_number();
                Value::Bool(res)
            }
            // This arm should never be matched.
            _ => unreachable!(),
        };

        // push the calculated result back on stack
        self.push(result);
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), VMError> {
        loop {
            // This blocks executes only when this debug tracing feature is enabled.
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

            // First byte should be the instruction byte of the code
            let instruction_byte = self.chunk.code[self.ip_offset];
            // Increment instruction pointer after reading the byte
            self.ip_offset += 1;

            // Try to convert that byte to `OpCode` enum
            if let Ok(opcode) = OpCode::try_from(instruction_byte) {
                // Conversion successful. Match opcode with different arms
                // to execute instruction
                match opcode {
                    // It means this is final instruction in the byte code
                    // Print the final result
                    OpCode::OpReturn => {
                        let v = self.pop().ok_or(
                            self.construct_runtime_error(format_args!("Expected return opcode")),
                        )?;
                        println!("{}", v);
                        return Ok(());
                    }
                    // Read constant from the constant pool
                    OpCode::OpConstant => {
                        let constant = self.read_constant();
                        // Push that constant onto the stack
                        self.push(constant);
                    }
                    // Negate the top value
                    OpCode::OpNegate => {
                        let value = self
                            .pop()
                            .ok_or(self.construct_runtime_error(format_args!("Expected value.")))?;

                        if value.is_number() {
                            self.push(-value);
                        } else {
                            return Err(self.construct_runtime_error(format_args!(
                                "Operand must be a number."
                            )));
                        }
                    }
                    // Only match binary operators
                    OpCode::OpAdd
                    | OpCode::OpSubtract
                    | OpCode::OpMultiply
                    | OpCode::OpDivide
                    | OpCode::OpGreater
                    | OpCode::OpLess => self.binary_op(opcode)?,
                    OpCode::OpNil => {
                        self.push(Value::Nil);
                    }
                    OpCode::OpTrue => {
                        self.push(Value::Bool(true));
                    }
                    OpCode::OpFalse => {
                        self.push(Value::Bool(false));
                    }
                    OpCode::OpNot => {
                        let value = self
                            .pop()
                            .ok_or_else(|| {
                                let err_message = format_args!("Expected value on stack");
                                self.construct_runtime_error(err_message)
                            })
                            .and_then(|val| {
                                if val.is_bool() || val.is_nil() {
                                    Ok(val)
                                } else {
                                    let err_message =
                                        format_args!("Operand of ! operator should be a boolean");
                                    return Err(self.construct_runtime_error(err_message));
                                }
                            })?;

                        self.push(Value::from(value.is_falsey()));
                    }
                    OpCode::OpEqual => {
                        let a = self.pop().ok_or_else(|| {
                            let arguments = format_args!("Expected value on stack");
                            self.construct_runtime_error(arguments)
                        })?;
                        let b = self.pop().ok_or_else(|| {
                            let arguments = format_args!("Expected value on stack");
                            self.construct_runtime_error(arguments)
                        })?;

                        self.push(Value::Bool(a == b));
                    }
                }
            }
        }
    }

    fn construct_runtime_error(&mut self, arguments: Arguments) -> VMError {
        let message = format!("{}", arguments);
        let instruction_index = self.ip_offset.checked_sub(1).and_then(|idx| {
            if idx < self.chunk.lines.len() {
                Some(idx)
            } else {
                None
            }
        });

        let line_info = if let Some(idx) = instruction_index {
            self.chunk.lines[idx]
        } else {
            -1
        };

        let message = if line_info != -1 {
            format!("[line {}] in bytecode: {}", line_info, message)
        } else {
            format!(
                "[line unknown] in bytecode (VM IP:{}): {}",
                self.ip_offset, message
            )
        };

        self.reset_stack();

        VMError::RuntimeError(message)
    }
}
