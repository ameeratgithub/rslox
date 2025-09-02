use crate::{
    chunk::OpCode,
    compiler::{
        CompilationContext,
        errors::CompilerError,
        precedence::{ParseRule, Precedence},
    },
    scanner::token::TokenType,
};

impl<'a> CompilationContext<'a> {
    // Performs the logical 'AND' operation between two boolean values.
    pub(super) fn logical_and(&mut self, _: bool) -> Result<(), CompilerError> {
        // Left hand expression has already been evaluated and result would be on stack.
        // So if that result is false, just emit jump, as we don't need to evaluate the second condition.
        let end_jump = self.emit_jump(OpCode::OpJumpIfFalse as u8)?;
        // Pop the result from the stack
        self.emit_byte(OpCode::OpPop as u8)?;
        // Evaluate right hand expression with precedence of `And`
        self.parse_precedence(Precedence::And)?;

        // Calculate the jump distance. If first condition is false, it will jump over the bytes of subsequent conditions.
        self.patch_jump(end_jump)
    }
    // Performs the logical 'OR' operation between two boolean values.
    pub(super) fn logical_or(&mut self, _: bool) -> Result<(), CompilerError> {
        // Left expression got evaluated, and is on the stack.
        // If that left expression is false, we need to evaluate the right expression.
        let else_jump = self.emit_jump(OpCode::OpJumpIfFalse as u8)?;
        // If left expression is true, we'll need to jump straight to the 'then' block, without checking any other condition
        let end_jump = self.emit_jump(OpCode::OpJump as u8)?;
        // This will skip to the remaining expression, if the first expression is false.
        self.patch_jump(else_jump)?;
        // Pop the result of evaluation of expression from the stack
        self.emit_byte(OpCode::OpPop as u8)?;
        // Parse the right hand side with `Precedence::Or`
        self.parse_precedence(Precedence::Or)?;
        // Patch the jump to go to the 'then' block.
        self.patch_jump(end_jump)
    }

    /// Writes byte code for binary instructions
    pub(super) fn binary(&mut self, _: bool) -> Result<(), CompilerError> {
        // Get binary operator
        let operator = self.get_previous_token_ty()?;

        // Get the rule of operator
        let rule = ParseRule::get_parse_rule(operator);

        // Recursive call parse_precedence if some high priority operator should be
        // executed first. Priority is increased via `precedence + 1`. If next operator doesn't
        // have higher precedence, only prefix rule will get called and then function will return
        self.parse_precedence(Precedence::from((rule.precedence as u8) + 1))?;

        // Check which binary operator is this, and emit byte code accordingly
        match operator {
            TokenType::Plus => self.emit_byte(OpCode::OpAdd as u8)?,
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract as u8)?,
            TokenType::Star => self.emit_byte(OpCode::OpMultiply as u8)?,
            TokenType::Slash => self.emit_byte(OpCode::OpDivide as u8)?,
            TokenType::BangEqual => self.emit_bytes(OpCode::OpEqual as u8, OpCode::OpNot as u8)?,
            TokenType::EqualEqual => self.emit_byte(OpCode::OpEqual as u8)?,
            TokenType::Greater => self.emit_byte(OpCode::OpGreater as u8)?,
            TokenType::GreaterEqual => {
                self.emit_bytes(OpCode::OpLess as u8, OpCode::OpNot as u8)?
            }
            TokenType::Less => self.emit_byte(OpCode::OpLess as u8)?,
            TokenType::LessEqual => {
                self.emit_bytes(OpCode::OpGreater as u8, OpCode::OpNot as u8)?
            }
            // There isn't any other binary operator allowed
            _ => unreachable!(),
        }

        Ok(())
    }

    /// Emits byte code for supported unary operators
    pub(super) fn unary(&mut self, _: bool) -> Result<(), CompilerError> {
        // Get operator
        let operator = self.get_previous_token_ty()?;

        // Recursive call to get the operand
        // In normal case, bytes for the Number operand will get emitted
        self.parse_precedence(Precedence::Unary)?;

        match operator {
            // Writes byte code (OpNot) for bang operator,
            TokenType::Bang => self.emit_byte(OpCode::OpNot as u8)?,
            // Writes byte code (OpNegate) for minus operator,
            TokenType::Minus => self.emit_byte(OpCode::OpNegate as u8)?,
            // There is no unary operator other than Minus, in this language
            // So unary function shouldn't be called if the operator is other
            // than Minus
            _ => unreachable!(),
        }

        Ok(())
    }
}
