/// This module handles all the stuff a VM is supposed to do
/// It takes source code, compiles it, gets bytecode (stored in chunk) from compiler
/// and then execute that bytecode
mod call_frame;
mod debug;
pub mod errors;
mod functions;
mod garbage_collection;
mod native;
mod operations;
mod variables;

use std::collections::HashMap;

use crate::{
    chunk::OpCode,
    constants::FRAMES_MAX,
    value::{Value, objects::ObjectNode},
    vm::{
        call_frame::CallFrame,
        errors::VMError,
        native::{clock_native, println},
    },
};

/// Data structure to handle a stack based virtual machine
pub struct VM {
    /// Stack to handle variables. Fixed stack size for simplicity, but has some limitations
    pub stack: Vec<Value>,
    /// A linked list to track Objects stored on heap, mainly used for garbage collection. Linked list is not the best data structure used for garbage collection. Just keeping it simple for now.
    pub objects: ObjectNode,
    /// A Datastructure, also known as `HashTable`, to store global variables for faster insertion and lookup.
    globals: HashMap<String, Value>,
    pub frames: Vec<CallFrame>,
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    /// Returns a new instance of the VM
    #[must_use]
    pub fn new() -> Self {
        Self {
            // All values should be nil/empty by default
            stack: Vec::new(),
            // No objects when vm is initialized
            objects: None,
            // No global variables when vm is initialized.
            globals: HashMap::new(),
            frames: Vec::with_capacity(FRAMES_MAX),
        }
    }

    /// Compiles source code, gets bytecode from compiler, and executes that bytecode
    /// # Errors
    ///
    /// Returns `VMError` if there's any runtime error
    pub fn interpret(&mut self) -> Result<(), VMError> {
        self.define_native("clock", clock_native)?;
        self.define_native("println", println)?;
        self.run()
    }

    pub fn replace_or_push(&mut self, value: Value, index: usize) {
        if self.stack.len() <= index {
            self.push(value);
        } else {
            self.stack[index] = value;
        }
    }
    // Push the value to stack, and increments the top
    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    // Pop the value from stack, and decrements the top
    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    pub fn current_frame(&mut self) -> &mut CallFrame {
        let top_index = self.frames.len() - 1;
        &mut self.frames[top_index]
    }

    /// # Errors
    ///
    /// Returns `VMError` if there's any runtime error
    pub fn run(&mut self) -> Result<(), VMError> {
        loop {
            #[cfg(feature = "debug_trace_execution")]
            self.debug();

            let instruction_byte = self.current_frame().read_byte();
            // Try to convert that byte to `OpCode` enum
            if let Ok(opcode) = OpCode::try_from(instruction_byte) {
                // Conversion successful. Match opcode with different arms
                // to execute instruction
                match opcode {
                    // It means this is final instruction in the byte code
                    OpCode::OpReturn => {
                        // If it's end of bytecode, just return.
                        if self.op_return() {
                            return Ok(());
                        }
                    }
                    // Usually used for expression statements. These statements may produce a result but this result will be popped because expression statements are only used for side effects.
                    OpCode::OpPop => {
                        self.pop().ok_or_else(||
                            // Return error if value on stack is not found
                            self.construct_runtime_error(format_args!("Expected value on the stack")))?;
                    }
                    OpCode::OpPrint => {
                        let v = self.pop().ok_or_else(||
                            // Return error if value on stack is not found
                            self.construct_runtime_error(format_args!("Expected value on the stack")))?;
                        print!("{v}");
                    }
                    OpCode::OpGetLocal => self.op_get_local(),
                    OpCode::OpSetLocal => self.op_set_local(),
                    OpCode::OpDefineGlobal => self.op_define_global()?,
                    OpCode::OpGetGlobal => self.op_get_global()?,
                    OpCode::OpSetGlobal => self.op_set_global()?,
                    // Read constant from the constant pool
                    OpCode::OpConstant => {
                        // Get constant value from constant pool
                        let constant = self.current_frame().read_constant();
                        // Push that constant onto the stack
                        self.push(constant);
                    }
                    // Negate the top value
                    OpCode::OpNegate => {
                        self.op_negate()?;
                    }
                    // Only match binary operators
                    // These all needs two number operands, so these are combined
                    // in a separate function
                    OpCode::OpAdd
                    | OpCode::OpSubtract
                    | OpCode::OpMultiply
                    | OpCode::OpDivide
                    | OpCode::OpGreater
                    | OpCode::OpLess => self.binary_op(&opcode)?,

                    // Push `Nil` onto the stack
                    OpCode::OpNil => {
                        self.push(Value::new_nil());
                    }

                    // Push true onto the stack
                    OpCode::OpTrue => {
                        self.push(true.into());
                    }

                    // Push false onto the stack
                    OpCode::OpFalse => {
                        self.push(false.into());
                    }

                    // Handles '!' operation
                    OpCode::OpNot => self.op_not()?,
                    // Compares two values
                    OpCode::OpEqual => self.op_equal()?,
                    OpCode::OpJumpIfFalse => {
                        // Reads the two bytes of distance being jumped
                        let offset = self.current_frame().read_u16();
                        // Result of the condition
                        let if_condition = &self.stack[self.stack.len() - 1];
                        // If condition is false, then perform the jump, other wise continue executing the statements
                        if if_condition.clone().is_falsey() {
                            self.current_frame().ip_offset += offset as usize;
                        }
                    }
                    OpCode::OpJump => {
                        // Read distance to jump
                        let offset = self.current_frame().read_u16();
                        // We don't check condition before jumping because else doesn't have any condition. If this instruction gets executed, just perform jump. When generating bytecode for if condition, when if condition is false, jump has to be immediately after this opcode (total 3 bytes). Otherwise it will get messy.
                        self.current_frame().ip_offset += offset as usize;
                    }
                    OpCode::OpLoop => {
                        let offset = self.current_frame().read_u16();
                        self.current_frame().ip_offset -= offset as usize;
                    }
                    OpCode::OpCall => self.op_call()?,
                }
            }
        }
    }
}
