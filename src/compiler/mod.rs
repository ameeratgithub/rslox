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

#[derive(Debug)]
pub enum CompilerError {
    ParserError(ParserError),
    ExpressionError(String),
}

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

pub struct Compiler<'a> {
    source: &'a str,
    parser: Parser<'a>,
    chunk: &'a mut Chunk,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str, chunk: &'a mut Chunk) -> Self {
        let scanner: Scanner<'_> = Scanner::new(source);
        let parser = Parser::new(scanner);

        Self {
            parser,
            chunk,
            source,
        }
    }

    pub fn compile(&mut self) -> Result<(), CompilerError> {
        // Consume first token
        self.parser.advance();

        self.expression()?;

        self.parser
            .consume(TokenType::Eof, "Expected end of expression")
            .map_err(|e| CompilerError::ParserError(e))?;

        self.end_compiler();

        Ok(())
    }

    fn expression(&mut self) -> Result<(), CompilerError> {
        self.parse_precedence(Precedence::Assignment)?;
        Ok(())
    }

    fn number(&mut self) -> Result<(), CompilerError> {
        let token = self.parser.previous.as_ref().unwrap();

        let val = &self.source[token.start..token.start + token.length as usize];

        let val: Value = val.parse().map_err(|e: ParseFloatError| {
            CompilerError::ParserError(ParserError::TokenError(e.to_string()))
        })?;

        self.emit_constant(val);
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

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), CompilerError> {
        // Parser already advanced one time, so this is second advance call
        self.parser.advance();
        // prev_ty should be a number. current_ty should be an operator

        if let Some(prefix_rule) =
            ParseRule::get_parse_rule(self.parser.previous.as_ref().unwrap().ty).prefix
        {
            prefix_rule(self)?;

            while precedence as u8
                <= ParseRule::get_parse_rule(self.parser.current.as_ref().unwrap().ty).precedence
                    as u8
            {
                self.parser.advance();

                if let Some(infix_rule) =
                    ParseRule::get_parse_rule(self.parser.previous.as_ref().unwrap().ty).infix
                {
                    infix_rule(self)?;
                }
            }
        } else {
            return Err(CompilerError::ExpressionError(
                "Expected expression".to_owned(),
            ));
        }
        Ok(())
    }

    fn binary(&mut self) -> Result<(), CompilerError> {
        // Safe to unwrap
        let operator = self.parser.previous.as_ref().unwrap().ty;

        let rule = ParseRule::get_parse_rule(operator);

        self.parse_precedence(Precedence::from((rule.precedence as u8) + 1))?;

        match operator {
            TokenType::Plus => self.emit_byte(OpCode::OpAdd as u8),
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract as u8),
            TokenType::Star => self.emit_byte(OpCode::OpMultiply as u8),
            TokenType::Slash => self.emit_byte(OpCode::OpDivide as u8),
            // There isn't any other binary operator allowed
            _ => unreachable!(),
        }

        Ok(())
    }

    fn unary(&mut self) -> Result<(), CompilerError> {
        // Safe to unwrap
        let operator = self.parser.previous.as_ref().unwrap().ty;

        // Recursive call to get the operand
        self.parse_precedence(Precedence::Unary)?;

        match operator {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate as u8),
            // There is no unary operator other than Minus, in this language
            // So unary function shouldn't be called if the operator is other
            // than Minus
            _ => unreachable!(),
        }

        Ok(())
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::OpConstant as u8, constant);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.chunk.add_constant(value);

        if constant > u8::MAX as usize {
            eprintln!("Too many constants in one chunk.");
            return 0;
        }

        constant as u8
    }

    fn end_compiler(&mut self) {
        self.emit_return();

        #[cfg(feature = "debug_trace_execution")]
        Debug::dissassemble_chunk(&self.chunk, "code");
    }

    fn emit_byte(&mut self, byte: u8) {
        // Getting parser.previous should always return a token.
        // So it's safe to unwrap
        let line = self.parser.previous.as_ref().unwrap().line;
        self.chunk.write_chunk(byte, line);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn as u8);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }
}
