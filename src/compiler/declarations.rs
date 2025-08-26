use crate::{
    chunk::OpCode,
    compiler::{CompilationContext, errors::CompilerError},
    scanner::token::TokenType,
};

impl<'a> CompilationContext<'a> {
    /// Responsible to handle all top level statements and declarations
    pub(super) fn declaration(&mut self) -> Result<(), CompilerError> {
        if self.match_curr_ty(TokenType::Fun)? {
            self.fun_declaration()?;
        }
        // If current token type is var, emit bytecode for variable declaration, otherwise proceed with other types of statements
        else if self.match_curr_ty(TokenType::Var)? {
            // If token is variable declaration, generate bytecode to declare the variable
            self.var_declaration()?;
        } else {
            // Generate bytecode to process the statement
            self.statement()?;
        }

        Ok(())
    }

    fn fun_declaration(&mut self) -> Result<(), CompilerError> {
        let global = self.parse_variable("Expected function name")?;
        self.mark_initialized();
        self.compile_function()?;
        self.define_variable(global)
    }

    /// Generates bytecode to declare a variable
    pub(super) fn var_declaration(&mut self) -> Result<(), CompilerError> {
        // Get the index of variable name, stored in constant pool
        let global = self.parse_variable("Expected variable name")?;
        if self.match_curr_ty(TokenType::Equal)? {
            // Current token is equal, evaluate the expression on the right hand side, which will be pushed on VM's stack
            self.expression()?;
        } else {
            // No value has been assigned to the variable. Assign `Nil` by default, which will be pushed on VM's stack
            self.emit_byte(OpCode::OpNil as u8)?;
        }
        // Variable declaration and initialization has been parsed. Consume ';' from the end.
        self.consume(TokenType::Semicolon, "Expected ';'")?;

        // Define global variable
        self.define_variable(global)?;

        Ok(())
    }
}
