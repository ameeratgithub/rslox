use crate::scanner::{
    Scanner, ScannerError,
    token::{Token, TokenType},
};

/// Collection of errors related to Parser
#[derive(Debug)]
pub enum ParserError {
    ScannerError(ScannerError),
    TokenError(String),
}

/// Implementation of Display trait to display errors nicely
impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ScannerError(error) => {
                write!(f, "{}", error)
            }
            Self::TokenError(error) => {
                write!(f, "{}", error)
            }
        }
    }
}

/// Data structure to hold `Token`s and `Scanner` to scan tokens
pub struct Parser<'a> {
    scanner: Scanner<'a>,
    pub current: Option<Token>,
    pub previous: Option<Token>,
}

impl<'a> Parser<'a> {
    /// Returns a fresh instance of the scanner
    pub fn new(scanner: Scanner<'a>) -> Self {
        Self {
            scanner,
            current: None,
            previous: None,
        }
    }
    /// Consumes the token, keeps track of past token and current token
    pub fn advance(&mut self) -> Result<(), ParserError> {
        // Assigns value to `self.previous`, we need `self.current` if error occures, so we
        // can't use `self.current.take()` to replace value of `self.current` by `None`
        self.previous = self.current.clone();

        // If token is alright, loop breaks, because it scans on demand
        // If there's an error scanning token, it continues and display errors.
        loop {
            match self.scanner.scan_token() {
                Ok(token) => {
                    self.current = Some(token);
                    break;
                }
                Err(e) => {
                    return Err(self.error_at_current(&format!("{}", e)));
                }
            }
        }

        Ok(())
    }

    /// Utility function to conditionaly consume token if it matches with desired
    /// `TokenType`
    pub fn consume(&mut self, other_ty: TokenType, message: &str) -> Result<(), ParserError> {
        let token = self.current.clone().ok_or(ParserError::TokenError(format!(
            "Expected token: {other_ty:?}, Found `None`"
        )))?;

        if token.ty == other_ty {
            self.advance()?;
            return Ok(());
        }

        Err(self.error_at_current(message))
    }

    pub fn error_at_previous(&self, message: &str) -> ParserError {
        // Safe to unwrap `previous` because value is present
        self.construct_error(&self.previous.as_ref().unwrap(), message)
    }

    fn error_at_current(&self, message: &str) -> ParserError {
        // Safe to unwrap `current` because value is present.
        self.construct_error(&self.current.as_ref().unwrap(), message)
    }

    fn construct_error(&self, token: &Token, message: &str) -> ParserError {
        let mut err_msg = String::from("");
        err_msg.push_str(&format!("[line {}] Error", token.line));

        if token.ty == TokenType::Eof {
            err_msg.push_str(" at end");
        } else if token.ty == TokenType::Error {
            // todo! revisit if we really need this token type
            // C implementation is different and that's not how we handle errors in Rust
        } else {
            let token_str = &self.scanner.source[token.start..token.start + token.length as usize];
            err_msg.push_str(&format!(" at '{}'", token_str));
        }

        err_msg.push_str(&format!(": {}\n", message));
        ParserError::TokenError(err_msg)
    }
}
