use crate::{
    chunk::OpCode,
    compiler::{CompilationContext, errors::CompilerError},
    scanner::token::TokenType,
};

impl CompilationContext<'_> {
    pub(super) fn begin_scope(&mut self) {
        self.compiler_mut().scope_depth += 1;
    }

    pub(super) fn end_scope(&mut self) -> Result<(), CompilerError> {
        self.compiler_mut().scope_depth -= 1;

        while !self.compiler().locals.is_empty()
            && self.compiler().locals[self.compiler().locals.len() - 1].depth
                > self.compiler().scope_depth
        {
            self.emit_byte(OpCode::OpPop as u8)?;
            // self.compiler_mut().local_count -= 1;
            self.compiler_mut().locals.pop();
        }

        Ok(())
    }

    pub(super) fn block(&mut self) -> Result<(), CompilerError> {
        while !self.check_current(TokenType::RightBrace) && !self.check_current(TokenType::Eof) {
            self.declaration()?;
        }

        self.consume(TokenType::RightBrace, "Expected '}' after block.")
    }
}
