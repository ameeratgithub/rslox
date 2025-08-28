use crate::{
    chunk::OpCode,
    compiler::{CompilationContext, Local, errors::CompilerError},
    constants::UINT8_COUNT,
    scanner::token::{Token, TokenType},
};

impl<'a> CompilationContext<'a> {
    /// Parses variable and generates bytecode for variable name, returns variable name's index of constant pool
    pub(super) fn parse_variable(&mut self, message: &str) -> Result<u8, CompilerError> {
        // Identifier, variable name in this case, would be consumed.
        self.consume(TokenType::Identifier, message)?;
        // After consumption, variable name is in previous token
        let prev_token = self
            .parser
            .previous
            .clone()
            .ok_or_else(|| self.construct_token_error(false, "Expected variable name"))?;

        self.declare_local_variable()?;

        if self.compiler().scope_depth > 0 {
            // Dummy table index
            return Ok(0);
        }

        // Generate bytecode for identifier token
        self.identifier_constant(&prev_token)
    }

    fn declare_local_variable(&mut self) -> Result<(), CompilerError> {
        if self.compiler().scope_depth == 0 {
            return Ok(());
        }

        let error = self.construct_token_error(false, "Variable name expected.");

        let name = self.parser.previous.clone().ok_or(error)?;

        for local in self.compiler().locals.iter().rev() {
            let scope_depth = self.compiler().scope_depth;
            // let local = &self.compiler().locals[i as usize];
            if local.depth != -1 && local.depth < scope_depth {
                break;
            }
            // let local_name = &local.name.clone();
            if self.are_identifiers_equal(&name, &local.name) {
                return Err(self.construct_token_error(
                    false,
                    "Already a variable with this name in this scope.",
                ));
            }
        }
        self.add_local_variable(name)?;
        Ok(())
    }

    fn are_identifiers_equal(&self, token_a: &Token, token_b: &Token) -> bool {
        if token_a.length != token_b.length {
            return false;
        }
        token_a.as_str(self.source) == token_b.as_str(self.source)
    }

    fn resolve_local(&mut self, name: &Token) -> Result<i32, CompilerError> {
        for (i, local) in self.compiler().locals.iter().enumerate() {
            // let local = &self.compiler().locals[i as usize];
            if self.are_identifiers_equal(name, &local.name) {
                if local.depth == -1 {
                    return Err(self.construct_token_error(
                        false,
                        "Can't read local variable in its own initializer",
                    ));
                }
                return Ok(i as i32);
            }
        }
        Ok(-1)
    }

    fn add_local_variable(&mut self, name: Token) -> Result<(), CompilerError> {
        if self.compiler().locals.len() == UINT8_COUNT as usize {
            return Err(self.construct_token_error(false, "Too many local variables in scope"));
        }

        let local = Local { name, depth: -1 };
        // let index = self.compiler().locals.len() as usize;
        self.compiler_mut().locals.push(local);
        // self.compiler_mut().local_count += 1;

        Ok(())
    }

    pub(super) fn mark_initialized(&mut self) {
        if self.compiler().scope_depth == 0 {
            return;
        }

        let scope_depth = self.compiler().scope_depth;
        let index = (self.compiler().locals.len() - 1) as usize;
        let local = &mut self.compiler_mut().locals[index];
        local.depth = scope_depth;
    }

    /// Writes bytecode to define variable
    pub(super) fn define_variable(&mut self, global: u8) -> Result<(), CompilerError> {
        if self.compiler().scope_depth > 0 {
            self.mark_initialized();
            return Ok(());
        }
        // Emits opcode and index of global variable
        self.emit_bytes(OpCode::OpDefineGlobal as u8, global)
    }

    /// Evaluates the variable declaration and initialization
    pub(super) fn variable(&mut self, can_assign: bool) -> Result<(), CompilerError> {
        let error = self.construct_token_error(false, "Expected previous token");
        let prev_token = self.parser.previous.as_ref().ok_or(error)?.clone();
        self.named_variable(&prev_token, can_assign)
    }

    fn named_variable(&mut self, name: &Token, can_assign: bool) -> Result<(), CompilerError> {
        let get_opcode: OpCode;
        let set_opcode: OpCode;

        let arg = self.resolve_local(name)?;
        let variable_offset;
        if arg != -1 {
            // It's a local variable. `arg` is offset/index in `locals` vector 
            variable_offset = arg as u8;
            get_opcode = OpCode::OpGetLocal;
            set_opcode = OpCode::OpSetLocal;
        } else {
            variable_offset = self.identifier_constant(name)?;
            get_opcode = OpCode::OpGetGlobal;
            set_opcode = OpCode::OpSetGlobal;
        }

        if can_assign && self.match_curr_ty(TokenType::Equal)? {
            // Current variable can assign, and current token is `Equal`, evaluate the expression on the right
            self.expression()?;
            // Emit the OpCode to set global variable, alongside the variable name index.
            self.emit_bytes(set_opcode as u8, variable_offset)
        } else {
            // Can't assign, or current token is not `Equal`, parse it as reading the global variable
            self.emit_bytes(get_opcode as u8, variable_offset)
        }
    }
}
