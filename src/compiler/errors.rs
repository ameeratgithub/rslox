use crate::compiler::{CompilationContext, parser::ParserError};

/// Custom Errors for compiler
#[derive(Debug)]
pub enum CompilerError {
    ParserError(ParserError),
    ExpressionError(String),
    ChunkError,
}

/// impl `Display` trait to show error nicely on console.
impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParserError(error) => {
                write!(f, "{error}")
            }
            Self::ExpressionError(error) => {
                write!(f, "{error}")
            }
            Self::ChunkError => {
                write!(f, "Chunk not found for current function")
            }
        }
    }
}

impl CompilationContext<'_> {
    pub(super) fn construct_token_error(
        &mut self,
        is_current: bool,
        message: &str,
    ) -> CompilerError {
        let error = if is_current {
            self.parser.error_at_current(message)
        } else {
            self.parser.error_at_previous(message)
        };
        CompilerError::ParserError(error)
    }
}
