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
pub enum OpCode {
    OpReturn = 0,
    OpConstant = 1,
}

impl TryFrom<u8> for OpCode {
    type Error = ChunkError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OpReturn),
            1 => Ok(Self::OpConstant),
            _ => Err(ChunkError::InvalidOpCode(value)),
        }
    }
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<u32>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            constants: vec![],
            lines: vec![],
        }
    }

    pub fn write_chunk(&mut self, byte: u8, line: u32) {
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
