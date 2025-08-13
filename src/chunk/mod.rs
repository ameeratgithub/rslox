/// This module handles operation codes for the vm
/// It's the byte representation of code for VM to execute
use crate::value::Value;

#[derive(Debug)]
pub enum ChunkError {
    InvalidOpCode(u8),
}

/// #[repr(u8)] tells that each `OpCode`'s variant should take only one byte, consistently
/// across all platforms.
/// This enum represents instructions, and each instruction should be of 1 byte, as of now, that's
/// why this is representation, and later, casting, is important.
#[repr(u8)]
#[derive(Debug, PartialEq)]
/// You can omit values (like 0, 1, 2), but it makes it clear and more readable what value an `OpCode` has.
pub enum OpCode {
    OpReturn = 0,
    OpConstant = 1,
    OpNegate = 2,
    OpAdd = 3,
    OpSubtract = 4,
    OpMultiply = 5,
    OpDivide = 6,
    OpNil = 7,
    OpTrue = 8,
    OpFalse = 9,
    OpNot = 10,
    OpEqual = 11,
    OpGreater = 12,
    OpLess = 13,
}

/// We need to convert `u8` to `OpCode`. Implementing `TryFrom` makes sense because `u8` can
/// have value for which OpCode doesn't exist
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
            _ => Err(ChunkError::InvalidOpCode(value)),
        }
    }
}

/// This actually is a data structure to handle a series of bytes
/// Can have different fields and associated functions to store bytes
pub struct Chunk {
    /// Code stored on a chunk. It's the read-only part
    pub code: Vec<u8>,
    /// List of constants defined in the code.
    pub constants: Vec<Value>,
    /// line number of code byte being written
    pub lines: Vec<i32>,
}

/// Implements functions for `Chunk`
impl Chunk {
    /// Returns fresh instance of `Chunk` struct
    pub fn new() -> Self {
        Self {
            code: vec![],
            constants: vec![],
            lines: vec![],
        }
    }

    /// Just push byte to the code vector, alongside the line number
    pub fn write_chunk(&mut self, byte: u8, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    /// Adds constant to constants pool
    /// Returns the index of constant in the pool
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
