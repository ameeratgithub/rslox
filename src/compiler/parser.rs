use crate::scanner::{
    Scanner, ScannerError,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub enum ParserError {
    ScannerError(ScannerError),
    TokenError(String),
}

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

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    pub current: Option<Token>,
    pub previous: Option<Token>,
    had_error: bool,
    panic_mode: bool,
}

impl<'a> Parser<'a> {
    pub fn new(scanner: Scanner<'a>) -> Self {
        Self {
            scanner,
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
        }
    }
    pub fn advance(&mut self) {
        // Assigns value to `self.previous`, and `self.current` will be replaced by `None`
        self.previous = self.current.take();

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
