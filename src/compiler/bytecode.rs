use crate::{
    chunk::OpCode,
    compiler::{CompilationContext, errors::CompilerError},
    scanner::token::Token,
    value::Value,
};

impl<'a> CompilationContext<'a> {
    /// Gets the variable name from source code and adds that name into constant pool of bytecode
    pub(super) fn identifier_constant(&mut self, name: &Token) -> Result<u8, CompilerError> {
        // Get name of the variable from source code and store as a string
        let name = name.as_str(self.source);
        // Make constant from variable name and get the index
        let constant_index = self.make_constant(name.into())?;
        Ok(constant_index)
    }

    /// Write a constant instruction and its index/offset in constant pool of the `chunk`
    pub(super) fn emit_constant(&mut self, value: Value) -> Result<(), CompilerError> {
        // Add value to the constant pool and get the index
        let constant = self.make_constant(value)?;
        // Emit store bytecode for OpCode and offset/index of constant in constant pool
        self.emit_bytes(OpCode::OpConstant as u8, constant)?;
        Ok(())
    }

    pub(super) fn emit_jump(&mut self, instruction: u8) -> Result<usize, CompilerError> {
        self.emit_byte(instruction)?;
        self.emit_byte(0xff)?;
        self.emit_byte(0xff)?;
        // Will point to first byte after instruction
        let offset = self.compiler_mut().chunk().code.len() - 2;
        Ok(offset)
    }

    pub(super) fn emit_loop(&mut self, loop_start: usize) -> Result<(), CompilerError> {
        self.emit_byte(OpCode::OpLoop as u8)?;
        let offset = self.compiler_mut().chunk().code.len() - loop_start + 2;
        if offset > u16::MAX as usize {
            let err = self.construct_token_error(false, "Loop body too large");
            return Err(err);
        }
        let offset_bytes = u16::to_be_bytes(offset as u16);
        self.emit_byte(offset_bytes[0])?;
        self.emit_byte(offset_bytes[1])
    }

    pub(super) fn patch_jump(&mut self, offset: usize) -> Result<(), CompilerError> {
        // Offset is first byte after `OpIfFalse` instruction, excluding 'then' block
        // `chunk.code` contains bytecode after executing 'then' block
        // So if failed, we want to jump to after 'then' block
        // -2 is important to calculate relative distance.
        // Consider following scenario:
        // 1. `if` instruction index: 9, code length: 10
        // 2. Two place holder bytes emitted, code length: 12
        // 3. Offset = code length - 2 = 10, which is first byte after `if` instruction
        // 4. `then` block compiled, let's say code length = 50
        // 5. code length - offset = 50 - 10 = 40
        // 6. offset is of first byte after instruction, so these are included right now in jump position calculation
        // 7. if code length is 50, then our then block should be of 38 bytes. Why? our code length was 12 when two place holder bytes were emitted. 12 + 38 = 50.
        // 8. to correctly calculate that jump position, we also need to subtract 2 from code length
        let jump = self.compiler_mut().chunk().code.len() - offset - 2;
        if jump > (u16::MAX as usize) {
            return Err(self.construct_token_error(false, "Too much code to jump over"));
        }
        // Jump is 32-bit, so we want to extract 2nd least significant byte.
        // jump>>8 will discard the least-significant byte and will make 2nd least significant, a least significant one.
        // Because our result is in least significant byte now, we will 'mask' our byte, by making essentialy all other bytes, zeros.
        let jump_bytes = (jump as u16).to_be_bytes();
        self.compiler_mut().chunk_mut().code[offset] = jump_bytes[0];
        // We've used our 2nd least significant byte, so we'll use least significant byte. It's already least significant, no need to right shift. Just set all other bytes to zeros, by masking.
        self.compiler_mut().chunk_mut().code[offset + 1] = jump_bytes[1];
        Ok(())
    }

    /// Adds constant to constant pool and returns its index
    pub(super) fn make_constant(&mut self, value: Value) -> Result<u8, CompilerError> {
        let constant = self.compiler_mut().chunk_mut().add_constant(value);
        // Only allows 256 constants to be stored in constant pool
        if constant > u8::MAX as usize {
            return Err(self.construct_token_error(false, "Too many constants in one chunk"));
        }
        Ok(constant as u8)
    }
    /// Writes a byte to the `chunk`
    pub(super) fn emit_byte(&mut self, byte: u8) -> Result<(), CompilerError> {
        let error = self.construct_token_error(false, "Expected token");
        let line = self.parser.previous.as_ref().ok_or(error)?.line;
        // Add byte with token's line
        self.compiler_mut().chunk_mut().write_chunk(byte, line);
        Ok(())
    }

    /// Writes OpReturn instruction at the end of the bytecode
    pub(super) fn emit_return(&mut self) -> Result<(), CompilerError> {
        self.emit_byte(OpCode::OpNil as u8)?;
        self.emit_byte(OpCode::OpReturn as u8)
    }

    /// Simply writes 2 bytes in order
    pub(super) fn emit_bytes(&mut self, byte1: u8, byte2: u8) -> Result<(), CompilerError> {
        self.emit_byte(byte1)?;
        self.emit_byte(byte2)?;
        Ok(())
    }
}
