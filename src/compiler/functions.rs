use crate::{
    chunk::OpCode,
    compiler::{CompilationContext, errors::CompilerError, types::FunctionType},
    scanner::token::TokenType,
    value::objects::FunctionObject,
};

impl<'a> CompilationContext<'a> {
    pub(super) fn compile_function(&mut self) -> Result<(), CompilerError> {
        let mut fun_ty = FunctionType::default_function();
        let mut fun_obj: FunctionObject = fun_ty.into();
        // Safe to unwrap
        fun_obj.name = Some(self.parser.previous.as_ref().unwrap().as_str(self.source));
        fun_ty = fun_obj.into();

        let child_compiler = super::CompilerState::new(fun_ty);
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

        let constant = self.make_constant(function_object)?;
        self.emit_bytes(OpCode::OpConstant as u8, constant)
    }

    pub(super) fn arguments_list(&mut self) -> Result<u8, CompilerError> {
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

    pub(super) fn call(&mut self, _: bool) -> Result<(), CompilerError> {
        let arg_count = self.arguments_list()?;
        self.emit_bytes(OpCode::OpCall as u8, arg_count)
    }
}
