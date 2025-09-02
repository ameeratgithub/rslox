use crate::scanner::{
    Scanner,
    errors::ScannerError,
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
                write!(f, "{error}")
            }
            Self::TokenError(error) => {
                write!(f, "{error}")
            }
        }
    }
}

#[derive(Clone)]
/// Data structure to hold `Token`s and `Scanner` to scan tokens
pub struct Parser<'a> {
    /// Scanner object to scan tokens on demand
    scanner: Scanner<'a>,
    /// Holds the current parsed token
    pub current: Option<Token>,
    /// Holds the previously parsed token. One step behind the current token.
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

        match self.scanner.scan_token() {
            // Token is valid, updated the current token
            Ok(token) => {
                self.current = Some(token);
                Ok(())
            }
            Err(e) => {
                // Return error with proper information
                Err(self.error_at_current(&format!("{e}")))
            }
        }
    }

    /// Utility function to conditionaly consume token if it matches with desired
    /// `TokenType`
    pub fn consume(&mut self, other_ty: TokenType, message: &str) -> Result<(), ParserError> {
        let token = self.current.clone().ok_or(ParserError::TokenError(format!(
            "Expected token: {other_ty:?}, Found `None`"
        )))?;

        // Token matches, consume token and return early.
        if token.ty == other_ty {
            self.advance()?;
            return Ok(());
        }

        // Return proper error message
        Err(self.error_at_current(message))
    }

    /// This returns error for previous token
    pub fn error_at_previous(&self, message: &str) -> ParserError {
        // Safe to unwrap `previous` because value is present
        self.construct_error(self.previous.as_ref().unwrap(), message)
    }

    /// This returns error for current token
    pub fn error_at_current(&self, message: &str) -> ParserError {
        // Safe to unwrap `current` because value is present.
        self.construct_error(self.current.as_ref().unwrap(), message)
    }

    /// This method is important because it formats error nicely with line numbers
    fn construct_error(&self, token: &Token, message: &str) -> ParserError {
        let mut err_msg = String::from("");
        // Get line information from token and add to the message
        err_msg.push_str(&format!("[line {}] Error", token.line));

        // Check if we've reached at the end
        if token.ty == TokenType::Eof {
            // Tell in the message that we've reached at the end
            err_msg.push_str(" at end");
        } else if token.ty == TokenType::Error {
            // todo! revisit if we really need this token type
            // C implementation is different and that's not how we handle errors in Rust
        } else {
            // Gets invalid/problematic token and append to the error message
            err_msg.push_str(&format!(" at '{}'", token.as_str(self.scanner.source)));
        }
        // Push the custom message at the end
        err_msg.push_str(&format!(": {message}\n"));
        // Return token error with formatted message
        ParserError::TokenError(err_msg)
    }
}
