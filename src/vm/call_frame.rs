use crate::value::Value;

pub struct CallFrame {
    pub(super) function: Value,
    pub(super) ip_offset: usize,
    pub(super) starting_offset: usize, // slots: Vec<Value>,
}

impl CallFrame {
    pub fn new(function: Value, ip_offset: usize, starting_offset: usize) -> Self {
        Self {
            function,
            ip_offset,
            starting_offset,
        }
    }

    pub(super) fn read_byte(&mut self) -> u8 {
        // First byte should be the instruction byte of the code
        let instruction_byte = self.function.as_function_ref().chunk.code[self.ip_offset];
        // Increment instruction pointer after reading the byte
        self.ip_offset += 1;

        instruction_byte
    }

    pub(super) fn read_u16(&mut self) -> u16 {
        // Read bytes
        let bytes = &self.function.as_function_ref().chunk.code[self.ip_offset..self.ip_offset + 2];
        // Advance two bytes
        self.ip_offset += 2;
        // Convert to u16
        u16::from_be_bytes([bytes[0], bytes[1]])
    }

    /// Reads constant from constant pool
    pub(super) fn read_constant(&mut self) -> Value {
        // We don't directly store constants on bytecode. Bytecode has the
        // index/offset of constant. We get that index from bytecode.
        let constant_position = self.function.as_function_ref().chunk.code[self.ip_offset];
        // Gets the value from constant pool.
        // This is not to be used in production. `constant_position` implies that there
        // would be maximum 256 constants, which should not be the case.
        // Multi-byte operations needed to be introduced to handle that
        let constant: Value =
            self.function.as_function_ref().chunk.constants[constant_position as usize].clone();
        // increment instruction pointer by 1, because we've consumed 1 byte
        self.ip_offset += 1;
        // return the value
        constant
    }
}
