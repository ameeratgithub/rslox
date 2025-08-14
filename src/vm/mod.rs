/// This module handles all the stuff a VM is supposed to do
/// It takes source code, compiles it, gets bytecode (stored in chunk) from compiler
/// and then execute that bytecode
pub mod constants;
use std::{fmt::Arguments, ptr::NonNull};

/// A custom `feature` to enable execution tracing.
/// When enabled, instructions are printed to console to see how bytecode is working
#[cfg(feature = "debug_trace_execution")]
use crate::debug::Debug;

use crate::{
    chunk::{Chunk, OpCode},
    compiler::CompilerError,
    value::{Object, ObjectNode, Value},
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
    chunk: &'a Chunk,
    /// Instruction pointer offset.
    ip_offset: usize,
    /// Stack to handle variables. Fixed stack size for simplicity, but has some limitations
    stack: [Value; STACK_MAX as usize],
    /// A pointer to check where we're on our stack. If value is 0, stack is empty.
    stack_top: usize,
    /// A linked list to track Objects stored on heap, mainly used for garbage collection
    pub objects: ObjectNode,
}

impl<'a> VM<'a> {
    /// Returns a new instance of the VM
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            ip_offset: 0,
            // All values should be nil/empty by default
            stack: [const { Value::new_nil() }; STACK_MAX as usize],
            stack_top: 0,
            // No objects when vm is just initialized
            objects: None,
        }
    }

    /// Compiles source code, gets bytecode from compiler, and executes that bytecode
    pub fn interpret(&mut self) -> Result<(), VMError> {
        // Run the bytecode (`self.chunk`) received from compiler.
        self.run()
    }

    pub fn reset_vm(&mut self) {
        #[cfg(feature = "debug_trace_execution")]
        self.display_garbage_items();
        // Remove items from garbage collection
        self.free_objects();
        // Reset stack to its initial state
        self.reset_stack();
    }

    /// Empties the stack and resets the top to '0'
    pub fn reset_stack(&mut self) {
        self.stack = [const { Value::new_nil() }; STACK_MAX as usize];
        self.stack_top = 0;
    }

    pub fn remove_object_pointer(&mut self, other: &NonNull<Object>) {
        while let Some(obj) = self.objects {
            if obj.eq(other) {
                self.objects = None;
            } else {
                unsafe {
                    let next = (*obj.as_ptr()).next;
                    if let Some(ptr) = next
                        && ptr.ne(other)
                    {
                        self.objects = next;
                        break;
                    }

                    self.objects = None;
                }
            }
        }
    }

    /// Responsible for freeing the memory allocated by runtime objects, such as string
    pub fn free_objects(&mut self) {
        // Iterate over the list of objects
        while let Some(obj) = self.objects {
            // Unsafe is required to dereference the raw pointer
            unsafe {
                // Assign `next` node to `self.objects`
                self.objects = (*obj.as_ptr()).next;
                // `Box` will automatically free the memory
                // Only free after pointing `self.objects` to `next` of current object
                // Otherwise `self.objects` will point to freed memory
                let _ = Box::from_raw(obj.as_ptr());
            }
        }
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
        Some(self.stack[self.stack_top].clone())
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
        let constant: Value = self.chunk.constants[constant_position as usize].clone();
        // increment instruction pointer by 1, because we've consumed 1 byte
        self.ip_offset += 1;
        // return the value
        constant
    }

    fn concatenate_strings(
        &mut self,
        left_operand: Value,
        right_operand: Value,
    ) -> Result<(), VMError> {
        // Convert `Value` to an `Object` of type `String`
        let left = if left_operand.is_object_string() {
            let left_pointer = left_operand.as_object_ref();
            self.remove_object_pointer(left_pointer);
            left_operand.as_object_string()
        } else {
            left_operand.as_literal_string()
        };

        let right = if right_operand.is_object_string() {
            let right_pointer = right_operand.as_object_ref();
            self.remove_object_pointer(right_pointer);
            right_operand.as_object_string()
        } else {
            right_operand.as_literal_string()
        };

        // Because it's a runtime operation, being executed by vm, it needs to create a value
        // by using special functions. This is important for garbage collection.
        let value = Value::from_runtime_str(left + &right, self)
            .map_err(|err| self.construct_runtime_error(format_args!("{}", err)))?;
        self.push(value);

        // Return because our work here is done.
        return Ok(());
    }

    // Performs the binary operation based on `opcode`.
    // `binary_op` should only be called when `opcode` supports binary operation.
    fn binary_op(&mut self, opcode: OpCode) -> Result<(), VMError> {
        // We're reading from left to right. So left operand got pushed first, then the right
        // operand got pushed. Let's say we're evaluating following expression
        // `2 - 1`
        // 1. 2 got pushed -> [2]
        // 2. 1 got pushed -> [2,1]
        // 3. 1 is right operand, and will be popped first, because it's on top
        // 4. 2 is left operand, and it will be popped second.
        // 5. so the correct operation will be left_operand - right_operand
        let right_operand = self
            .pop()
            .ok_or_else(|| {
                // If value isn't on stack, throw an error
                let err = format_args!("Expected value on stack");
                self.construct_runtime_error(err)
            })
            // This will get executed if value is on stack
            .and_then(|val| {
                // We're only interested if right operand is a number or a string
                if val.is_number() || val.is_string() {
                    Ok(val)
                } else {
                    // If right operand is not a number, return an error
                    let err = format_args!("Expected number or string as right operand");
                    Err(self.construct_runtime_error(err))
                }
            })?;
        let left_operand = self
            .pop()
            .ok_or_else(|| {
                // If value isn't on stack, throw an error
                let err = format_args!("Expected value on stack");
                self.construct_runtime_error(err)
            })
            // This will get executed if value is on stack
            .and_then(|val| {
                let operands_are_numbers = right_operand.is_number() && val.is_number();
                let operands_are_strings = right_operand.is_string() && val.is_string();
                // We're only interested if both operands are numbers or both are strings
                if operands_are_numbers || (operands_are_strings && opcode == OpCode::OpAdd) {
                    Ok(val)
                } else {
                    // Invalid operation on operands, return error
                    let err = format_args!("Expected both operands to be of same type");
                    Err(self.construct_runtime_error(err))
                }
            })?;

        // Concatinate If object is string
        if right_operand.is_string() && left_operand.is_string() {
            return self.concatenate_strings(left_operand, right_operand);
        }

        // Match the opcode and perform the relevant operation
        let result = match opcode {
            // Works because `Add` trait is implemented
            OpCode::OpAdd => left_operand + right_operand,
            // Works because `Sub` trait is implemented
            OpCode::OpSubtract => left_operand - right_operand,
            // Works because `Mul` trait is implemented
            OpCode::OpMultiply => left_operand * right_operand,
            // Works because `Div` trait is implemented
            OpCode::OpDivide => left_operand / right_operand,
            // Checks if left > right
            OpCode::OpGreater => {
                // We've checked that both operands are numbers, so we can safely
                // convert them for comparison
                let res = left_operand.to_number() > right_operand.to_number();
                res.into()
            }
            // Checks if left < right
            OpCode::OpLess => {
                // We've checked that both operands are numbers, so we can safely
                // convert them for comparison
                let res = left_operand.to_number() < right_operand.to_number();
                res.into()
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
                        let v = self.pop().ok_or_else(||
                            // Return error if OpReturn code not found
                            self.construct_runtime_error(format_args!("Expected return opcode")))?;
                        // Print calculated result at the end of the execution
                        println!("======  Result  ======");
                        println!("{}", v);
                        println!("======================");
                        return Ok(());
                    }
                    // Read constant from the constant pool
                    OpCode::OpConstant => {
                        // Get constant value from constant pool
                        let constant = self.read_constant();
                        // Push that constant onto the stack
                        self.push(constant);
                    }
                    // Negate the top value
                    OpCode::OpNegate => {
                        let value = self.pop().ok_or_else(||
                            // Return error if value isn't on stack
                            self.construct_runtime_error(format_args!("Expected an operand.")))?;

                        // Operand should be a number
                        if value.is_number() {
                            // This should work because we've implemented `Neg` trait
                            self.push(-value);
                        } else {
                            // Return error if operand isn't a number
                            return Err(self.construct_runtime_error(format_args!(
                                "Operand must be a number."
                            )));
                        }
                    }
                    // Only match binary operators
                    // These all needs two number operands, so these are combined
                    // in a separate function
                    OpCode::OpAdd
                    | OpCode::OpSubtract
                    | OpCode::OpMultiply
                    | OpCode::OpDivide
                    | OpCode::OpGreater
                    | OpCode::OpLess => self.binary_op(opcode)?,

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
                    OpCode::OpNot => {
                        let value = self
                            .pop()
                            .ok_or_else(|| {
                                // If stack is empty, return error
                                let err_message = format_args!("Expected value on stack");
                                self.construct_runtime_error(err_message)
                            })
                            .and_then(|val| {
                                // Value should be bool or nil
                                if val.is_bool() || val.is_nil() {
                                    Ok(val)
                                } else {
                                    // Since we can't negate other types, return error
                                    let err_message = format_args!(
                                        "Operand of ! operator should be a bool or nil"
                                    );
                                    return Err(self.construct_runtime_error(err_message));
                                }
                            })?;

                        // This negates original value and pushes onto the stack
                        self.push(Value::from(value.is_falsey()));
                    }
                    // Compares two values
                    OpCode::OpEqual => {
                        let a = self.pop().ok_or_else(|| {
                            // Return error if stack is empty
                            let arguments = format_args!("Expected value on stack");
                            self.construct_runtime_error(arguments)
                        })?;
                        let b = self.pop().ok_or_else(|| {
                            // Return error if stack is empty
                            let arguments = format_args!("Expected value on stack");
                            self.construct_runtime_error(arguments)
                        })?;
                        // This is possible because of PartialEq trait implementation
                        self.push((a == b).into());
                    }
                }
            }
        }
    }

    /// Show items in garbadge collection
    #[cfg(feature = "debug_trace_execution")]
    pub fn display_garbage_items(&mut self) {
        println!("====== Garbage Collection Items ======");
        if self.objects.is_some() {
            // Temporary variable to hold objects
            let mut head = self.objects;
            // Execute till the end of the list
            while let Some(obj) = head {
                // Required to access to dereference the raw pointer
                unsafe {
                    // Dereference the object from raw pointer
                    print!("{}", (*obj.as_ptr()));
                    // Assign next object to current list
                    head = (*obj.as_ptr()).next;
                }

                // There is a next object, show arrow to look like it's pointing to next element
                if head.is_some() {
                    print!(" -> ")
                } else {
                    // Break the line
                    println!()
                }
            }
        } else {
            println!("-> No Item Found")
        }

        println!("======================================");
    }

    /// This is important because we want to display errors nicely.
    /// It gets dynamic arguments, and constructs proper error
    fn construct_runtime_error(&mut self, arguments: Arguments) -> VMError {
        // Instruction is one step behind the current offset, so subtracting 1
        let instruction_index = self.ip_offset.checked_sub(1).and_then(|idx| {
            // We want to get line number from index so this check is important
            if idx < self.chunk.lines.len() {
                Some(idx)
            } else {
                None
            }
        });

        let line_info = if let Some(idx) = instruction_index {
            // Get line number of the current instruction
            self.chunk.lines[idx]
        } else {
            -1
        };

        let message = format!("{}", arguments);
        let message = if line_info != -1 {
            // If line number exists, show line number with message
            format!("[line {}] in bytecode: {}", line_info, message)
        } else {
            // Invalid line number, show ip_offset
            format!(
                "[line unknown] in bytecode (VM IP:{}): {}",
                self.ip_offset, message
            )
        };

        // Error occured, reset stack.
        self.reset_vm();

        // Return proper error
        VMError::RuntimeError(message)
    }
}
