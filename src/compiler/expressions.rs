use crate::{
    compiler::{
        CompilationContext,
        errors::CompilerError,
        precedence::{ParseRule, Precedence},
    },
    scanner::token::TokenType,
};

impl<'a> CompilationContext<'a> {
    pub(super) fn expression(&mut self) -> Result<(), CompilerError> {
        // Parse expression based on precedence
        self.parse_precedence(Precedence::Assignment)?;
        Ok(())
    }

    pub(super) fn grouping(&mut self, _: bool) -> Result<(), CompilerError> {
        // Initial '(' has already been consumed, so next we have to evaluate inner expression.
        // Recursive call to evaluate the inner expression
        self.expression()?;

        // When inner expression is evaluated/parsed, consume the right parenthesis
        self.consume(TokenType::RightParen, "Expected ')' after expression.")?;

        Ok(())
    }

    /// Executes instructions according to precedence.
    pub(super) fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), CompilerError> {
        // Parser already advanced one time, so this is second advance call
        // So in the case of `2+1`, parser would be at `+`
        self.parser.advance().map_err(CompilerError::ParserError)?;

        // Check if previous token has any prefix rule
        if let Some(prefix_rule) = ParseRule::get_parse_rule(self.get_previous_token_ty()?).prefix {
            // `can_assign` is used in `prefix_rule` of variables. It is being passed to other rules, infix and prefix, as well but it's being ignored there. This rule should be executed with `can_assign=true` when a variable is declared AND initialized. If it's not initialized, there's no assignment (`TokenType::Equal`) operator, and expression method shouldn't be called.
            let can_assign = precedence as u8 <= Precedence::Assignment as u8;
            // Prefix rule in an expression gets executed first
            prefix_rule(self, can_assign)?;

            // Repeat while precedence is lower than current token
            while precedence as u8
                <= ParseRule::get_parse_rule(self.get_current_token_ty()?).precedence as u8
            {
                // Consume token to get right operand
                self.parser.advance().map_err(CompilerError::ParserError)?;

                // It's the same operator who's precedence got compared.
                // After calling advance, it becomes previous token
                if let Some(infix_rule) =
                    ParseRule::get_parse_rule(self.get_previous_token_ty()?).infix
                {
                    // If operator has infix rule, execute it
                    infix_rule(self, can_assign)?;
                }

                // After the infix rule, like expression `a * b`, there shouldn't be any equal sign or `can_assign` should be false. This throws error when we right something like `a * b = c + d;`
                if can_assign && self.match_curr_ty(TokenType::Equal)? {
                    return Err(CompilerError::ExpressionError(
                        "Invalid assignment target".to_owned(),
                    ));
                }
            }
        } else {
            // Token should have an infix rule
            return Err(self.construct_token_error(false, "Expected expression."));
        }

        Ok(())
    }
}
