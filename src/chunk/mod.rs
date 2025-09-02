/// This module handles operation codes for the vm
/// It's the byte representation of code for VM to execute
use crate::value::Value;

#[derive(Debug)]
/// Error thrown when invalid opcode gets detected.
pub enum ChunkError {
    InvalidOpCode(u8),
}

/// #[repr(u8)] tells that each `OpCode`'s variant should take only one byte, consistently across all platforms.
/// This enum represents instructions, and each instruction should be of 1 byte, as of now, that's why this is representation, and later, casting, is important.
#[repr(u8)]
#[derive(Debug, PartialEq)]
/// You can omit values (like 0, 1, 2), but it makes it clear and more readable what value an `OpCode` has.
pub enum OpCode {
    /// Should only be added at the end of the bytecode.
    /// Takes a single byte in bytecode.
    OpReturn = 0,
    /// Indicates that a constant needs to be read from bytecode and added on stack.
    /// Takes 2 bytes: 1 for opcode and 1 for position in constant pool.
    OpConstant = 1,
    /// Indicates that right operand should be negated. Only valid for numeric values.
    /// Takes a single byte. Expects value to be already at the top of the stack.
    OpNegate = 2,
    /// Pops two values from the stack, performs addition, and pushes the result back onto the stack. Only valid for numbers and strings.
    /// Takes a single byte. Just pops two values from the stack.
    OpAdd = 3,
    /// Pops two values from the stack, performs subtraction, and pushes the result back onto the stack. Only valid for numbers.
    /// Takes a single byte. Just pops two values from the stack.
    OpSubtract = 4,
    /// Pops two values from the stack, performs multiplication, and pushes the result back onto the stack. Only valid for numbers.
    /// Takes a single byte. Just pops two values from the stack.
    OpMultiply = 5,
    /// Pops two values from the stack, performs division, and pushes the result back onto the stack. Only valid for numbers.
    /// Takes a single byte. Just pops two values from the stack.
    OpDivide = 6,
    /// Pushes `Nil`, a literal value, onto the stack. Takes a single byte.
    OpNil = 7,
    /// Pushes `True`, a literal value, onto the stack. Takes a single byte.
    OpTrue = 8,
    /// Pushes `False`, a literal value, onto the stack. Takes a single byte,
    OpFalse = 9,
    /// Pop a value from the stack, if value is truthy, inverts it, and push it back onto the stack.
    /// Takes a single byte. Expects value to be ready for popped.
    OpNot = 10,
    /// Pops two values from the stack, performs comparison, and pushes the result back onto the stack.
    /// Takes a single byte.
    OpEqual = 11,
    /// Pops two values from the stack, checks if left value is greater than right value, and pushes the result back onto the stack.
    /// Takes a single byte.
    OpGreater = 12,
    /// Pops two values from the stack, checks if left value is less than right value, and pushes the result back onto the stack.
    /// Takes a single byte.
    OpLess = 13,
    /// Pops the value from the stack, and print that value to the console.
    /// Takes a single byte.
    OpPrint = 14,
    /// Simply pops the value from the stack. Takes a single byte.
    OpPop = 15,
    /// Reads name of the variable from bytecode, gets value from bytecode, inserts variable name and value into a hashmap, called `globals`
    /// Takes 2 bytes: 1 for `OpCode`, 1 to store position of variable in constant pool.
    OpDefineGlobal = 16,
    /// Reads name of the variable from bytecode, gets value from the hashmap.
    /// Takes 2 bytes: 1 for `OpCode`, 1 is the position of the variable in the constant pool.
    OpGetGlobal = 17,
    /// Reads name of the variable from bytecode, gets value from the stack, and insert variable name and new value into `globals`.
    /// Takes 2 bytes: 1 for `OpCode`, 1 is the position of the variable in the constant pool.
    OpSetGlobal = 18,

    OpGetLocal = 19,
    OpSetLocal = 20,
    OpJumpIfFalse = 21,
    OpJump = 22,
    OpLoop = 23,
    OpCall = 24,
}

/// We need to convert `u8` to `OpCode`. Implementing `TryFrom` makes sense because `u8` can
/// have value for which `OpCode` doesn't exist
impl TryFrom<u8> for OpCode {
    type Error = ChunkError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OpReturn),
            1 => Ok(Self::OpConstant),
            2 => Ok(Self::OpNegate),
            3 => Ok(Self::OpAdd),
            4 => Ok(Self::OpSubtract),
            5 => Ok(Self::OpMultiply),
            6 => Ok(Self::OpDivide),
            7 => Ok(Self::OpNil),
            8 => Ok(Self::OpTrue),
            9 => Ok(Self::OpFalse),
            10 => Ok(Self::OpNot),
            11 => Ok(Self::OpEqual),
            12 => Ok(Self::OpGreater),
            13 => Ok(Self::OpLess),
            14 => Ok(Self::OpPrint),
            15 => Ok(Self::OpPop),
            16 => Ok(Self::OpDefineGlobal),
            17 => Ok(Self::OpGetGlobal),
            18 => Ok(Self::OpSetGlobal),
            19 => Ok(Self::OpGetLocal),
            20 => Ok(Self::OpSetLocal),
            21 => Ok(Self::OpJumpIfFalse),
            22 => Ok(Self::OpJump),
            23 => Ok(Self::OpLoop),
            24 => Ok(Self::OpCall),
            _ => Err(ChunkError::InvalidOpCode(value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// A data structure to handle the bytecode generated by compiler.
/// Can have different fields and associated functions to store bytes
pub struct Chunk {
    /// Bytecode stored on a chunk. It's the read-only part
    pub code: Vec<u8>,
    /// List of constants defined in the code.
    pub constants: Vec<Value>,
    /// line number of code byte being written
    pub lines: Vec<i32>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

/// Implements functions for `Chunk`
impl Chunk {
    /// Returns fresh instance of `Chunk`
    #[must_use]
    pub fn new() -> Self {
        Self {
            code: vec![],
            constants: vec![],
            lines: vec![],
        }
    }

    /// Adds byte to the code vector, alongside the line number
    pub fn write_chunk(&mut self, byte: u8, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    /// Adds constant to constant pool and returns the index of constant in the pool
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
