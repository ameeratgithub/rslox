use crate::scanner::{
    errors::ScannerError,
    token::{Token, TokenType},
};

pub mod errors;
mod identifier;
mod literals;
#[cfg(test)]
mod tests;
pub mod token;

#[derive(Clone)]
/// Data structure to scan the source code and return tokens
pub struct Scanner<'a> {
    /// Reference to the source code string
    pub source: &'a str,
    /// Starting position of the scanner
    start: usize,
    /// Current position of the scanner
    current: usize,
    /// Current line number
    line: i32,
}

impl<'a> Scanner<'a> {
    /// Returns the fresh instance of `Scanner`
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// Checks if the character is alphabetical
    /// Should start with capital or small letter or underscore
    /// Used to check first character for identifiers or keywords
    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    /// This scan token on demand, and returns a single token
    pub fn scan_token(&mut self) -> Result<Token, ScannerError> {
        // Ignore whitespaces at the start of the token
        self.skip_whitespace();
        // Starting from where previous token scan left.
        // Both should be 0 when scanning first token
        self.start = self.current;

        // If we've reached the end, just return `Eof` token
        if self.is_at_end() {
            return Ok(self.make_token(TokenType::Eof));
        }

        // Because we've checked that we're not at end of the file/source, it's safe to unwrap
        let character = self.advance().unwrap();

        // Return token identifier, if start of the lexeme is either an alphabet
        // or an underscore
        if self.is_alpha(character) {
            return Ok(self.identifier());
        }

        // Return token number, if start of the token is a digit
        if character.is_ascii_digit() {
            return Ok(self.number());
        }

        // Match characters to return relevant token
        let token = match character {
            // Single character tokens
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            // Single or possible double character tokens
            '!' => {
                let ty = self.pick_token_type('=', TokenType::BangEqual, TokenType::Bang);
                self.make_token(ty)
            }
            '=' => {
                let ty = self.pick_token_type('=', TokenType::EqualEqual, TokenType::Equal);
                self.make_token(ty)
            }
            '<' => {
                let ty = self.pick_token_type('=', TokenType::LessEqual, TokenType::Less);
                self.make_token(ty)
            }
            '>' => {
                let ty = self.pick_token_type('=', TokenType::GreaterEqual, TokenType::Greater);
                self.make_token(ty)
            }
            '"' => self.string()?,
            _ => {
                let err = ScannerError::UnexpectedCharacter {
                    line: self.line,
                    character,
                };

                return Err(err);
            }
        };

        Ok(token)
    }

    /// Skips/ignores whitespaces and consumes characters
    fn skip_whitespace(&mut self) {
        loop {
            // Take a look at current character
            if let Some(c) = self.peek() {
                match c {
                    // Just consume characters
                    ' ' | '\r' | '\t' => {
                        self.advance();
                    }
                    // Consume character and increment line number
                    '\n' => {
                        self.line += 1;
                        self.advance();
                    }
                    // Potential candidate for comment in code
                    '/' => {
                        // We need `peek_next()` because we're just looking at current character, and
                        // not advancing, looking at current character and looking to match next
                        // character would require `peek_next()`
                        if let Some(c) = self.peek_next()
                            && c == '/'
                        {
                            // Consume characters until a new line is found or we've reached at the end
                            while let Some(c) = self.peek()
                                && c != '\n'
                                && !self.is_at_end()
                            {
                                self.advance();
                            }
                        } else {
                            // next character is not '/', just ignore it and return
                            return;
                        }
                    }
                    _ => return,
                }
            } else {
                // No character found, just return from the function
                return;
            }
        }
    }

    /// Returns if `current` pointer has been reached at the end of the source code
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> Option<char> {
        // Not so efficient, but since chars can be multibyte due to utf8 support, it is
        // correct approach.
        // self.source[self.current..].chars().next()

        // This implementation assumes that there's only ASCII support in language
        // So we can return byte as character
        let byte = self.source.as_bytes().get(self.current).copied()?;
        Some(byte as char)
    }
    fn peek_next(&self) -> Option<char> {
        // Not so efficient, but since chars can be multibyte due to utf8 support, it is
        // correct approach.
        // self.source[self.current..].chars().next()

        // This implementation assumes that there's only ASCII support in language
        // So we can return byte as character
        let byte = self.source.as_bytes().get(self.current + 1).copied()?;
        Some(byte as char)
    }

    pub fn advance(&mut self) -> Option<char> {
        // This block supports utf8 characters, but is slower.
        {
            // let c = self.source[self.current..].chars().next()?;
            // self.current += c.len_utf8();
        }

        // This implementation assumes that there's only ASCII support in language
        // So we can return byte as character
        let byte = self.source.as_bytes().get(self.current).copied()?;
        self.current += 1;
        Some(byte as char)
    }

    /// Returns true if current character is matched with expected character
    fn match_char(&mut self, expected: char) -> bool {
        // Nothing to match, already at the end, return false
        if self.is_at_end() {
            return false;
        }

        // Check if character is equal to expected character
        if let Some(c) = self.peek()
            && c == expected
        {
            // Consume character to make token
            self.current += 1;
            return true;
        }

        false
    }

    /// Simple helper function to reduce boilerplate
    fn pick_token_type(&mut self, c: char, if_ty: TokenType, else_ty: TokenType) -> TokenType {
        if self.match_char(c) { if_ty } else { else_ty }
    }

    /// Makes a new token and return it
    fn make_token(&self, ty: TokenType) -> Token {
        Token::new(
            ty,
            self.start,
            (self.current - self.start) as u32,
            self.line,
        )
    }
}
