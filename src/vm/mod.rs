/// This module handles all the stuff a VM is supposed to do
/// It takes source code, compiles it, gets bytecode (stored in chunk) from compiler
/// and then execute that bytecode
pub mod constants;
use std::{collections::HashMap, fmt::Arguments, ptr::NonNull};

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
pub struct VM {
    /// A mutable reference to the `Chunk`.
    pub chunk: Chunk,
    /// Instruction pointer offset.
    ip_offset: usize,
    /// Stack to handle variables. Fixed stack size for simplicity, but has some limitations
    stack: [Value; STACK_MAX],
    /// A pointer to check where we're on our stack. If value is 0, stack is empty.
    stack_top: usize,
    /// A linked list to track Objects stored on heap, mainly used for garbage collection. Linked list is not the best data structure used for garbage collection. Just keeping it simple for now.
    pub objects: ObjectNode,
    /// A Datastructure, also known as HashTable, to store global variables for faster insertion and lookup.
    globals: HashMap<String, Value>,
}

impl VM {
    /// Returns a new instance of the VM
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            // Offset from where vm would start executing.
            ip_offset: 0,
            // All values should be nil/empty by default
            stack: [const { Value::new_nil() }; STACK_MAX],
            // This would be one step ahead of the current element.
            stack_top: 0,
            // No objects when vm is initialized
            objects: None,
            // No global variables when vm is initialized.
            globals: HashMap::new(),
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
        // All the values should be `Nil` by default
        self.stack = [const { Value::new_nil() }; STACK_MAX as usize];
        self.stack_top = 0;
    }

    /// This method iterates over linked list and remove a node if pointer matches. Useful method when extracting a value from a raw pointer and that raw pointer needs to be dropped.
    pub fn remove_object_pointer(&mut self, other: &NonNull<Object>) {
        // Tracks current node, starting from head
        let mut current = self.objects;
        // Tracks previous node
        let mut prev: Option<NonNull<Object>> = None;
        // Start iterating the list, starting from head
        while let Some(node) = current {
            if node.eq(other) {
                unsafe {
                    // Get the next pointer of the current node
                    let next = (*node.as_ptr()).next;
                    if let Some(mut prev_node) = prev {
                        // It isn't first node, set `prev.next` to `current.next`. It's like creating a link between nodes and removing itself from the middle
                        (*prev_node.as_mut()).next = next;
                    } else {
                        // It's the first node, remove first node by setting itself to next
                        self.objects = next;
                    }
                }
                // Node removed, return now.
                return;
            }
            // Match not found
            unsafe {
                // Set `prev` to `current`
                prev = current;
                // Move `current` to `next` node
                current = (*node.as_ptr()).next;
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

    /// This function concatenate strings and manage memory at runtime while doing so. If there are two literal strings in bytecode, concatenation will allocate memory for result, at runtime, and that value should be garbage collected
    fn concatenate_strings(
        &mut self,
        left_operand: Value,
        right_operand: Value,
    ) -> Result<(), VMError> {
        // Check if left_operand is heap allocated string
        let left = if left_operand.is_object_string() {
            // Get reference to the `ObjectPointer` of `left_operand`
            let left_pointer = left_operand.as_object_ref();
            // Remove that pointer from linked list, because `Value` is going to be extracted
            self.remove_object_pointer(left_pointer);
            // Extract string from the pointer
            left_operand.as_object_string()
        } else {
            // It's not heap allocated string, so just extract the value
            left_operand.as_literal_string()
        };

        // Check if right_operand is heap allocated string
        let right = if right_operand.is_object_string() {
            // Get reference to the `ObjectPointer` of `right_operand`
            let right_pointer = right_operand.as_object_ref();
            // Remove that pointer from linked list, because `Value` is going to be extracted
            self.remove_object_pointer(right_pointer);
            // Extract string from the pointer
            right_operand.as_object_string()
        } else {
            // It's not heap allocated string, so just extract the value
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

        // Concatinate if both operands are strings
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

    fn read_byte(&mut self) -> u8 {
        // First byte should be the instruction byte of the code
        let instruction_byte = self.chunk.code[self.ip_offset];
        // Increment instruction pointer after reading the byte
        self.ip_offset += 1;

        instruction_byte
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

            let instruction_byte = self.read_byte();

            // Try to convert that byte to `OpCode` enum
            if let Ok(opcode) = OpCode::try_from(instruction_byte) {
                // Conversion successful. Match opcode with different arms
                // to execute instruction
                match opcode {
                    // It means this is final instruction in the byte code
                    OpCode::OpReturn => {
                        // Exit interpreter
                        return Ok(());
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
                        println!("{}", v);
                    }
                    OpCode::OpGetLocal => {
                        let slot = self.read_byte();
                        self.push(self.stack[slot as usize].clone());
                    }
                    OpCode::OpSetLocal => {
                        let slot = self.read_byte();
                        self.stack[slot as usize] = self.stack[self.stack_top - 1].clone();
                    }
                    // Define a global variable and insert into `HashMap`
                    OpCode::OpDefineGlobal => {
                        // Read the variable name from bytecode and convert it to literal string
                        let name = self.read_constant().as_literal_string();
                        // If variable is not initilized, default value stored on stack should be `Nil`. In both cases, we're expecting value on the stack.
                        let value= self.pop().ok_or_else(||
                            // Return error if value on stack is not found
                            self.construct_runtime_error(format_args!("Expected value on the stack")))?;
                        // Insert variable's name and value into `HashMap`
                        self.globals.insert(name, value);
                    }
                    // Gets the value of variable and pushes onto the stack
                    OpCode::OpGetGlobal => {
                        // Read the variable name from bytecode and convert it to literal string
                        let name = self.read_constant().as_literal_string();
                        // Get the global variable from `HashMap`
                        let value = self.globals.get(&name).cloned().ok_or_else(|| {
                            // Variable doesn't exist. Return an error.
                            self.construct_runtime_error(format_args!(
                                "Undefined variable '{name}'"
                            ))
                        })?;
                        // Variable exists, push value on the stack for later use.
                        self.push(value);
                    }
                    // Sets value to already declared global variable
                    OpCode::OpSetGlobal => {
                        // Read the variable name from bytecode and convert it to literal string
                        let name = self.read_constant().as_literal_string();
                        // Check for underflow. If `stack_top` is less than zero after subtraction, return error
                        let value_index = self.stack_top.checked_sub(1).ok_or_else(|| {
                            self.construct_runtime_error(format_args!("Expected value on stack"))
                        })?;
                        // Clone value from the stack. We just want to store it in HashMap, so no need to pop or replace value.
                        let value = self.stack[value_index].clone();
                        // Check whether variable is defined or not
                        if !self.globals.contains_key(&name) {
                            // Variable has not been defined, return error.
                            return Err(self.construct_runtime_error(format_args!(
                                "Undefined variable '{}'",
                                name
                            )));
                        }
                        // Variable has been defined. Update it's value
                        self.globals.insert(name, value);
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
                    OpCode::OpJumpIfFalse => {
                        // Reads the two bytes of distance being jumped
                        let offset = self.read_u16();
                        // Result of the condition
                        let if_condition = &self.stack[self.stack_top - 1];
                        // If condition is false, then perform the jump, other wise continue executing the statements
                        if if_condition.clone().is_falsey() {
                            self.ip_offset += offset as usize;
                        }
                    }
                    OpCode::OpJump => {
                        // Read distance to jump
                        let offset = self.read_u16();
                        // We don't check condition before jumping because else doesn't have any condition. If this instruction gets executed, just perform jump. When generating bytecode for if condition, when if condition is false, jump has to be immediately after this opcode (total 3 bytes). Otherwise it will get messy.
                        self.ip_offset += offset as usize;
                    }
                    OpCode::OpLoop => {
                        let offset = self.read_u16();
                        self.ip_offset -= offset as usize;
                    }
                }
            }
        }
    }

    fn read_u16(&mut self) -> u16 {
        // Read bytes
        let bytes = &self.chunk.code[self.ip_offset..self.ip_offset + 2];
        // Advance two bytes
        self.ip_offset += 2;
        // Convert to u16
        u16::from_be_bytes([bytes[0], bytes[1]])
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
