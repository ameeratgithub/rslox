use std::num::ParseFloatError;

use crate::{
    chunk::OpCode,
    compiler::{errors::CompilerError, CompilationContext},
    scanner::token::TokenType,
    value::Value,
};

impl<'a> CompilationContext<'a> {
    pub(super) fn number(&mut self, _: bool) -> Result<(), CompilerError> {
        let error = self.construct_token_error(false, "Expected Number, found None");
        // Get previous token, which should be a number
        let token = self.parser.previous.as_ref().ok_or(error)?;
        // Extract number from source code.
        let val = &self.source[token.start..token.start + token.length as usize];
        // Try to parse number to the `Value`
        let val: f64 = val
            .parse()
            .map_err(|e: ParseFloatError| self.construct_token_error(false, &e.to_string()))?;

        // Write this in chunk
        self.emit_constant(val.into())?;

        Ok(())
    }

    /// Generates bytecode for keywords that generate literal values
    pub(super) fn literal(&mut self, _: bool) -> Result<(), CompilerError> {
        let operator = self.get_previous_token_ty()?;
        match operator {
            TokenType::False => self.emit_byte(OpCode::OpFalse as u8)?,
            TokenType::Nil => self.emit_byte(OpCode::OpNil as u8)?,
            TokenType::True => self.emit_byte(OpCode::OpTrue as u8)?,
            _ => unreachable!(),
        }

        Ok(())
    }

    pub(super) fn string(&mut self, _: bool) -> Result<(), CompilerError> {
        let error = self.construct_token_error(false, "Expected token");
        let token = self.parser.previous.as_ref().ok_or(error)?;
        // Skip the double quotes character '"'
        let start_index = token.start + 1;
        // Last index of token would be `length - 1`, and has ending double quotes
        // So, also skipping ending '"'
        let end_index = start_index + (token.length as usize - 2);
        // String value from source code is getting copied into virtual machine
        let str = self.source[start_index..end_index].to_owned();
        // Create a Value object from String
        let value = Value::from(str);
        // Emit that value as constant
        self.emit_constant(value)?;

        Ok(())
    }
}
