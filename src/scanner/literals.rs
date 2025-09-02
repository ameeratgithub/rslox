use crate::scanner::{
    Scanner,
    errors::ScannerError,
    token::{Token, TokenType},
};

impl Scanner<'_> {
    /// Consumes remaining characters of a number
    /// First digit would already been consumed when this called
    pub(super) fn number(&mut self) -> Token {
        //! Check if current character is digit. If it is, consume that character
        //! Run until non-digit character is encountered
        while let Some(c) = self.peek()
            && c.is_ascii_digit()
        {
            self.advance();
        }

        // Check if current character is a '.' and next character is a number
        if let Some(c) = self.peek()
            && c == '.'
            && let Some(ch) = self.peek_next()
            && ch.is_ascii_digit()
        {
            // Consume the '.' character
            self.advance();

            // Check the current character, and consume it if it's a digit
            // Repeat untile non-digit character is found
            while let Some(c) = self.peek()
                && c.is_ascii_digit()
            {
                self.advance();
            }
        }

        // Return the token of type `Number`
        self.make_token(TokenType::Number)
    }

    pub(super) fn string(&mut self) -> Result<Token, ScannerError> {
        // Consumed character is '"', now we need to peek if the current character is '"'
        // If peek doesn't return '"', just advance, otherwise return a String token because
        // current string has reached its end.
        while let Some(c) = self.peek()
            && c != '"'
            // This condition is important to avoid panic if string is not terminated at we're
            // already at the end of the source.
            && !self.is_at_end()
        {
            // This allows string to be multiline
            if let Some(c) = self.peek()
                && c == '\n'
            {
                self.line += 1;
            }
            // Consome character
            self.advance();
        }

        // Remember we didn't consume closing '"' of a string. If scanner's already at the end
        // then it's unterminated string.
        if self.is_at_end() {
            return Err(ScannerError::UnterminatedString { line: self.line });
        }

        // Consume closing '"'
        self.advance();

        // Make token of type `String` and return it
        Ok(self.make_token(TokenType::String))
    }
}
