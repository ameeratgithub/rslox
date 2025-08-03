///
/// This module is responsible of taking source code, parse it and generate bytecode
/// This is a single phase compiler. It means it parses code and and generate bytecode in one step
/// Byte code is generated as soon as an expression has been parsed
///
use std::num::ParseFloatError;

use crate::{
    chunk::{Chunk, OpCode},
    compiler::{
        parser::{Parser, ParserError},
        precedence::{ParseRule, Precedence},
    },
    scanner::{Scanner, token::TokenType},
    value::Value,
};

#[cfg(feature = "debug_trace_execution")]
use crate::debug::Debug;

pub mod parser;
pub mod precedence;

/// Custom Errors for compiler
#[derive(Debug)]
pub enum CompilerError {
    ParserError(ParserError),
    ExpressionError(String),
}

/// impl `Display` trait to show error nicely on console.
impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParserError(error) => {
                write!(f, "{}", error)
            }
            Self::ExpressionError(error) => {
                write!(f, "{}", error)
            }
        }
    }
}

/// Data structure that handles compiler functionality, which includes parsing and generating bytecode
/// Compiler doesn't care about how to execute bytecode, it's the responsibility of the virtual machine.
/// It scans tokens on demand, which can reduce memory usage.
pub struct Compiler<'a> {
    source: &'a str,
    parser: Parser<'a>,
    chunk: &'a mut Chunk,
}

impl<'a> Compiler<'a> {
    /// Returns a fresh instance of `Compiler`
    pub fn new(source: &'a str, chunk: &'a mut Chunk) -> Self {
        let scanner: Scanner<'_> = Scanner::new(source);
        // Parser needs to scan tokens on demand, it'll need scanner object for that
        let parser = Parser::new(scanner);

        Self {
            parser,
            chunk,
            source,
        }
    }

    // Responsible to generate byte code from source code
    pub fn compile(&mut self) -> Result<(), CompilerError> {
        // Consumes first token
        // Important because we look back and see previous tokens
        self.parser
            .advance()
            .map_err(|e| CompilerError::ParserError(e))?;

        // Start parsing expressions.
        self.expression()?;

        // At the end of the expression, there should be the token Eof
        // If it's not the Eof, then something is wrong with code
        self.parser
            .consume(TokenType::Eof, "Expected end of expression")
            .map_err(|e| CompilerError::ParserError(e))?;

        self.end_compiler()?;

        Ok(())
    }

    fn expression(&mut self) -> Result<(), CompilerError> {
        // Parse expression based on precedence
        self.parse_precedence(Precedence::Assignment)?;
        Ok(())
    }

    fn number(&mut self) -> Result<(), CompilerError> {
        // Get previous token, which should be a number
        let token = self
            .parser
            .previous
            .as_ref()
            .ok_or(CompilerError::ParserError(ParserError::TokenError(
                "Expected Number, found None".to_owned(),
            )))?;

        // Extract number from source code.
        let val = &self.source[token.start..token.start + token.length as usize];

        // Try to parse number to the `Value`
        let val: f64 = val.parse().map_err(|e: ParseFloatError| {
            CompilerError::ParserError(ParserError::TokenError(e.to_string()))
        })?;

        // Write this in chunk
        self.emit_constant(Value::Number(val))?;

        Ok(())
    }

    fn grouping(&mut self) -> Result<(), CompilerError> {
        // Initial '(' has already been consumed, so next we have to evaluate inner
        // expression.
        // Recursive call to evaluate the inner expression
        self.expression()?;

        // When inner expression is evaluated/parsed, consume the right parenthesis
        self.parser
            .consume(TokenType::RightParen, "Expected ')' after expression.")
            .map_err(|e| CompilerError::ParserError(e))?;

        Ok(())
    }

    /// Returns type of current token
    fn get_current_token_ty(&mut self) -> Result<TokenType, CompilerError> {
        Ok(self
            .parser
            .current
            .as_ref()
            .ok_or(CompilerError::ParserError(ParserError::TokenError(
                "Previous token not found".to_owned(),
            )))?
            .ty)
    }
    /// Returns type of previous token
    fn get_previous_token_ty(&mut self) -> Result<TokenType, CompilerError> {
        Ok(self
            .parser
            .previous
            .as_ref()
            .ok_or(CompilerError::ParserError(ParserError::TokenError(
                "Previous token not found".to_owned(),
            )))?
            .ty)
    }

    /// Executes instructions according to precedence.
    fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), CompilerError> {
        // Parser already advanced one time, so this is second advance call
        self.parser
            .advance()
            .map_err(|e| CompilerError::ParserError(e))?;

        // Check if previous token has any prefix rule to
        if let Some(prefix_rule) = ParseRule::get_parse_rule(self.get_previous_token_ty()?).prefix {
            // Prefix rule in an expression gets execute first
            prefix_rule(self)?;

            // Repeat while precedence is lower than current token
            while precedence as u8
                <= ParseRule::get_parse_rule(self.get_current_token_ty()?).precedence as u8
            {
                // Consume token to get right operand
                self.parser
                    .advance()
                    .map_err(|e| CompilerError::ParserError(e))?;

                // It's the same operator who's precedence got compared.
                // After calling advance, it becomes previous token
                if let Some(infix_rule) =
                    ParseRule::get_parse_rule(self.get_previous_token_ty()?).infix
                {
                    // If operator has infix rule, execute it
                    infix_rule(self)?;
                }
            }
        } else {
            // Token should have an infix rule
            let err = self.parser.error_at_previous("Expected expression.");
            return Err(CompilerError::ParserError(err));
        }

        Ok(())
    }

    /// Writes byte code for binary instructions
    fn binary(&mut self) -> Result<(), CompilerError> {
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
            TokenType::GreaterEqual => self.emit_bytes(OpCode::OpLess as u8, OpCode::OpNot as u8)?,
            TokenType::Less => self.emit_byte(OpCode::OpLess as u8)?,
            TokenType::LessEqual => self.emit_bytes(OpCode::OpGreater as u8, OpCode::OpNot as u8)?,
            // There isn't any other binary operator allowed
            _ => unreachable!(),
        }

        Ok(())
    }

    /// Emits byte code for supported unary operators
    fn unary(&mut self) -> Result<(), CompilerError> {
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

    fn literal(&mut self) -> Result<(), CompilerError> {
        let operator = self.get_previous_token_ty()?;
        match operator {
            TokenType::False => self.emit_byte(OpCode::OpFalse as u8)?,
            TokenType::Nil => self.emit_byte(OpCode::OpNil as u8)?,
            TokenType::True => self.emit_byte(OpCode::OpTrue as u8)?,
            _ => unreachable!(),
        }

        Ok(())
    }
    /// Write a constant instruction and its index/offset in constant pool of
    /// the `chunk`
    fn emit_constant(&mut self, value: Value) -> Result<(), CompilerError> {
        let constant = self.make_constant(value)?;
        self.emit_bytes(OpCode::OpConstant as u8, constant)?;
        Ok(())
    }

    /// Adds constant to constant pool and returns its index
    fn make_constant(&mut self, value: Value) -> Result<u8, CompilerError> {
        let constant = self.chunk.add_constant(value);

        // Only allows 256 constants to be stored in constant pool
        if constant > u8::MAX as usize {
            let err = self
                .parser
                .error_at_previous("Too many constants in one chunk");
            return Err(CompilerError::ParserError(err));
        }

        Ok(constant as u8)
    }

    /// Executes when all expressions are evaluated
    fn end_compiler(&mut self) -> Result<(), CompilerError> {
        self.emit_return()?;

        // Disassembles byte code to see what's going on
        #[cfg(feature = "debug_trace_execution")]
        Debug::dissassemble_chunk(&self.chunk, "code");

        Ok(())
    }

    /// Writes a byte to the `chunk`
    fn emit_byte(&mut self, byte: u8) -> Result<(), CompilerError> {
        // Getting parser.previous should always return a token.
        // So it's safe to unwrap
        let line = self
            .parser
            .previous
            .as_ref()
            .ok_or(CompilerError::ParserError(ParserError::TokenError(
                "Previous Token not found".to_owned(),
            )))?
            .line;
        self.chunk.write_chunk(byte, line);
        Ok(())
    }

    /// Writes OpReturn instruction at the end of the bytecode
    fn emit_return(&mut self) -> Result<(), CompilerError> {
        self.emit_byte(OpCode::OpReturn as u8)
    }

    /// Simply writes 2 bytes in order
    fn emit_bytes(&mut self, byte1: u8, byte2: u8) -> Result<(), CompilerError> {
        self.emit_byte(byte1)?;
        self.emit_byte(byte2)?;
        Ok(())
    }
}
