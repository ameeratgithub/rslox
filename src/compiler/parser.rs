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
    had_error: bool,
    panic_mode: bool,
}

impl<'a> Parser<'a> {
    /// Returns a fresh instance of the scanner
    pub fn new(scanner: Scanner<'a>) -> Self {
        Self {
            scanner,
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
        }
    }
    /// Consumes the token, keeps track of past token and current token
    pub fn advance(&mut self) {
        // Assigns value to `self.previous`, and `self.current` will be replaced by `None`
        self.previous = self.current.take();

        // If token is alright, loop breaks, because it scans on demand
        // If there's an error scanning token, it continues and display errors.
        loop {
            match self.scanner.scan_token() {
                Ok(token) => {
                    self.current = Some(token);
                    break;
                }
                Err(e) => {
                    eprintln!("{e}");
                    // todo! revisit these properties. I, currently, don't think we need them
                    self.had_error = true;
                    self.panic_mode = true;
                }
            }
        }
    }

    /// Utility function to conditionaly consume token if it matches with desired
    /// `TokenType`
    pub fn consume(&mut self, other_ty: TokenType, message: &str) -> Result<(), ParserError> {
        let token = self.current.clone().ok_or(ParserError::TokenError(format!(
            "Expected token: {other_ty:?}, Found `None`"
        )))?;


        if token.ty == other_ty {
            self.advance();
            return Ok(());
        }

        Err(ParserError::TokenError(format!(
            "{}, Found {:?}",
            message.to_owned(),
            token
        )))
    }
}
