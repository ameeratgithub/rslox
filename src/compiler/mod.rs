///
/// This module is responsible of taking source code, parse it and generate bytecode. This is a single phase compiler. It means it parses code and and generate bytecode in one step
/// Byte code is generated as soon as an expression has been parsed. This module shouldn't care about object values created at runtime. Like strings can be created at runtime and you can also specify a string as literal. They both should behave differently, and string created at runtime should be garbage collected.
///
use crate::{
    chunk::{Chunk, OpCode},
    compiler::{errors::CompilerError, parser::Parser, types::FunctionType},
    constants::UINT8_COUNT,
    scanner::{
        Scanner,
        token::{Token, TokenType},
    },
    value::{FunctionObject, Value},
};

#[cfg(feature = "debug_trace_execution")]
use crate::debug::Debug;

mod bytecode;
mod declarations;
pub mod errors;
mod expressions;
mod literals;
mod operations;
pub mod parser;
mod precedence;
mod statements;
#[cfg(test)]
mod tests;
pub mod types;
mod variables;

#[derive(Debug, Clone)]
pub struct Local {
    name: Token,
    depth: i32,
}

pub struct CompilationContext<'a> {
    /// Reference of the source code
    source: &'a str,
    /// Parser object to parse code on demand
    parser: Parser<'a>,
    stack: Vec<CompilerState>,
}

impl<'a> CompilationContext<'a> {
    pub fn new(source: &'a str) -> Self {
        let scanner: Scanner<'_> = Scanner::new(source);
        // Parser needs to scan tokens on demand, it'll need scanner object for that
        let parser = Parser::new(scanner);

        Self {
            stack: Vec::new(),
            source,
            parser,
        }
    }

    pub fn push(&mut self, compiler: CompilerState) {
        self.stack.push(compiler);
    }

    pub fn compiler(&self) -> &CompilerState {
        self.stack.last().expect("Compiler stack is empty")
    }

    pub fn compiler_mut(&mut self) -> &mut CompilerState {
        self.stack.last_mut().expect("Compiler stack is empty")
    }

    pub fn pop(&mut self) -> CompilerState {
        self.stack.pop().expect("Compiler stack is empty")
    }

    // Responsible to generate byte code from source code
    pub fn compile(&mut self) -> Result<Value, CompilerError> {
        // Consumes first token
        // Important because we look back and see previous tokens
        self.parser
            .advance()
            .map_err(|e| CompilerError::ParserError(e))?;
        // Iterate til the end of the file. If current token is `Eof`, loop will end.
        while !self.match_curr_ty(TokenType::Eof)? {
            // Process statements
            self.declaration()?;
        }

        self.end_compiler()
    }

    pub fn compile_function(&mut self) -> Result<(), CompilerError> {
        let mut fun_ty = FunctionType::default_function();
        let mut fun_obj: FunctionObject = fun_ty.into();
        // Safe to unwrap
        fun_obj.name = Some(self.parser.previous.as_ref().unwrap().as_str(self.source));
        fun_ty = fun_obj.into();

        let child_compiler = CompilerState::new(fun_ty);
        self.push(child_compiler);

        self.begin_scope();
        self.consume(TokenType::LeftParen, "Expected '(' after function name")?;

        if !self.check_current(TokenType::RightParen) {
            loop {
                let fun_ty = std::mem::replace(
                    &mut self.compiler_mut().function_type,
                    FunctionType::default_function(),
                );

                let mut fun_obj: FunctionObject = fun_ty.into();
                fun_obj.arity += 1;

                if fun_obj.arity > 255 {
                    return Err(
                        self.construct_token_error(true, "Can't have more than 255 parameters")
                    );
                }

                let _ = std::mem::replace(&mut self.compiler_mut().function_type, fun_obj.into());

                let constant = self.parse_variable("Expected parameter name")?;
                self.define_variable(constant)?;

                if !self.match_curr_ty(TokenType::Comma)? {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;
        self.consume(TokenType::LeftBrace, "Expected '{' before function bofy")?;
        self.block()?;

        let function_object = self.end_compiler()?;
        let value: Value = function_object.into();

        let constant = self.make_constant(value)?;
        self.emit_bytes(OpCode::OpConstant as u8, constant)
    }

    fn call(&mut self, _: bool) -> Result<(), CompilerError> {
        let arg_count = self.arguments_list()?;
        self.emit_bytes(OpCode::OpCall as u8, arg_count)
    }

    fn arguments_list(&mut self) -> Result<u8, CompilerError> {
        let mut arg_count = 0u8;

        if !self.check_current(TokenType::RightParen) {
            loop {
                self.expression()?;

                if arg_count == 255 {
                    return Err(
                        self.construct_token_error(false, "Can't have more than 255 arguments.")
                    );
                }

                arg_count += 1;

                if !self.match_curr_ty(TokenType::Comma)? {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expected ')' after arguments.")?;
        Ok(arg_count)
    }

    fn consume(&mut self, ty: TokenType, message: &str) -> Result<(), CompilerError> {
        self.parser
            .consume(ty, message)
            .map_err(|e| CompilerError::ParserError(e))
    }

    fn begin_scope(&mut self) {
        self.compiler_mut().scope_depth += 1;
    }

    fn end_scope(&mut self) -> Result<(), CompilerError> {
        self.compiler_mut().scope_depth -= 1;

        while self.compiler().locals.len() > 0
            && self.compiler().locals[(self.compiler().locals.len() - 1) as usize].depth
                > self.compiler().scope_depth
        {
            self.emit_byte(OpCode::OpPop as u8)?;
            // self.compiler_mut().local_count -= 1;
            self.compiler_mut().locals.pop();
        }

        Ok(())
    }

    fn block(&mut self) -> Result<(), CompilerError> {
        while !self.check_current(TokenType::RightBrace) && !self.check_current(TokenType::Eof) {
            self.declaration()?;
        }

        self.consume(TokenType::RightBrace, "Expected '}' after block.")
    }

    /// Returns type of the current token
    fn get_current_token_ty(&mut self) -> Result<TokenType, CompilerError> {
        let error = self.construct_token_error(true, "Expected token");
        Ok(self.parser.current.as_ref().ok_or(error)?.ty)
    }

    /// Returns type of the previous token
    fn get_previous_token_ty(&mut self) -> Result<TokenType, CompilerError> {
        let error = self.construct_token_error(false, "Expected token");
        Ok(self.parser.previous.as_ref().ok_or(error)?.ty)
    }

    /// Consume token if current token matches, and returns true. Otherwise returns false
    fn match_curr_ty(&mut self, ty: TokenType) -> Result<bool, CompilerError> {
        if !self.check_current(ty) {
            // Token doesn't match, return false
            return Ok(false);
        }
        // Token matches, consume token
        self.parser
            .advance()
            .map_err(|e| CompilerError::ParserError(e))?;
        Ok(true)
    }

    /// Checks if current token matches with desired token
    fn check_current(&self, ty: TokenType) -> bool {
        if let Some(token) = &self.parser.current {
            return token.ty == ty;
        }
        false
    }

    /// Executes when all expressions are evaluated
    fn end_compiler(&mut self) -> Result<Value, CompilerError> {
        self.emit_return()?;

        let func = &mut self.compiler_mut().function_type;
        let fun_type = std::mem::replace(func, FunctionType::default_script());

        let fun_obj: FunctionObject = fun_type.into();

        // Disassembles byte code to see what's going on
        #[cfg(feature = "debug_trace_execution")]
        {
            let name = if let Some(name) = &fun_obj.name {
                name
            } else {
                "<script>"
            };

            Debug::dissassemble_chunk(&self.compiler().chunk(), name);
        }

        self.pop();
        Ok(fun_obj.into())
    }
}

/// Data structure that handles compiler functionality, which includes parsing and generating bytecode
/// Compiler doesn't care about how to execute bytecode, it's the responsibility of the virtual machine.
/// It scans tokens on demand, which can reduce memory usage.
pub struct CompilerState {
    locals: Vec<Local>,
    // chunk: Chunk,
    // local_count: i32,
    scope_depth: i32,
    function_type: FunctionType,
}

impl CompilerState {
    /// Returns a fresh instance of `Compiler`
    pub fn new(function_type: FunctionType) -> Self {
        let locals = Vec::with_capacity(UINT8_COUNT);
        // let mut local_count: i32 = 0;

        // locals.push(Local {
        //     name: Token {
        //         ty: TokenType::String,
        //         start: 0,
        //         length: 0,
        //         line: 0,
        //     },
        //     depth: 0,
        // });

        // local_count += 1;

        Self {
            // chunk: Chunk::new(),
            locals,
            scope_depth: 0,
            // local_count,
            function_type,
        }
    }

    fn chunk(&self) -> &Chunk {
        match &self.function_type {
            FunctionType::Function(fun) => &fun.chunk,
            FunctionType::Script(script) => &script.chunk,
        }
    }

    fn chunk_mut(&mut self) -> &mut Chunk {
        match &mut self.function_type {
            FunctionType::Function(fun) => &mut fun.chunk,
            FunctionType::Script(script) => &mut script.chunk,
        }
    }
}
