use crate::scanner::token::{Token, TokenType};

pub mod token;

/// Represents all errors related to the scanner
#[derive(Debug)]
pub enum ScannerError {
    /// Unexpected/Unrecognized character alongside the line number
    UnexpectedCharacter { line: i32, character: char },
    /// Represents unterminated, which has no ending double quote '"', string error
    UnterminatedString { line: i32 },
}

/// Display trait implementation to print errors nicely
impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScannerError::UnexpectedCharacter { line, character } => {
                write!(
                    f,
                    "[line {}] Error: Unexpected character '{}'",
                    line, character
                )
            }
            ScannerError::UnterminatedString { line } => {
                write!(f, "[line {}] Error: Unterminated string.", line)
            }
        }
    }
}

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

    /// Consumes remaining characters of a number
    /// First digit would already been consumed when this called
    fn number(&mut self) -> Token {
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

    /// Determines the type of the identifier
    fn identifier_type(&self) -> TokenType {
        // Since we've already consumed at least one character, it's safe to unwrap here
        // as `self.start` is the starting index of token
        // We've to start from starting position of the token to identify the type
        // because at this point, `self.current` has been reached at the end of the token
        let starting_char = self.source[self.start..].chars().next().unwrap();
        match starting_char {
            // Checks for keyword 'and', first character has been consumed so start is 1
            // we need to look for 2 more characters, 'nd', hence the length of 2.
            // If match is successful, we will get the `TokenType::And` in return
            // If match is unsuccessful, we will get the default Identifier type `TokenType::Identifier`
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            // Checks for keyword 'class'
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            // Checks for keyword 'else'
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            // Checks for different possible keywords starting with 'f'
            'f' => {
                // It means more than 1 characters have been processed
                if self.current - self.start > 1 {
                    // Since more than 1 characters have been processed, it's save to unwrap second
                    // character.
                    let second_char = self.source[self.start + 1..].chars().next().unwrap();
                    // Keywords starting with 'f' can have one of 'a', 'o', 'u' as second character
                    // so, we'll try to match with pre-defined keywords.
                    match second_char {
                        // Checks for keyword `false`
                        'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                        // Checks for keyword `for`
                        'o' => self.check_keyword(2, 1, "r", TokenType::False),
                        // Checks for keyword `fun`
                        'u' => self.check_keyword(2, 1, "n", TokenType::False),
                        // It's a custom Identifier
                        _ => TokenType::Identifier,
                    }
                } else {
                    // Not a keyword. Custom Identifier
                    TokenType::Identifier
                }
            }
            // Checks for keyword `if`
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            // Checks for keyword `nil`
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            // Checks for keyword `or`
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            // Checks for keyword `print`
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            // Checks for keyword `return`
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            // Checks for keyword `super`
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            // Checks for multiple keywords starting with `t`
            't' => {
                if self.current - self.start > 1 {
                    // Since more than 1 characters have been processed, it's save to unwrap second
                    // character.
                    let second_char = self.source[self.start + 1..].chars().next().unwrap();
                    // Keywords starting with 't' can have one of 'h', 'r' as second character
                    // so, we'll try to match with pre-defined keywords.
                    match second_char {
                        // Checks for keyword `this`
                        'h' => self.check_keyword(2, 2, "is", TokenType::This),
                        // Checks for keyword `true`
                        'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                        // No keyword found. It's custom identifier
                        _ => TokenType::Identifier,
                    }
                } else {
                    // No keyword found. It's custom identifier
                    TokenType::Identifier
                }
            }
            // Checks for keyword `var`
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            // Checks for keyword `while`
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            // No keyword found. It's custom identifier
            _ => TokenType::Identifier,
        }
    }

    /// Returns the identifier
    fn identifier(&mut self) -> Token {
        //! Consume characters until current character is alphabetic or is a digit
        //! Remeber this function is called when first character is either a english alphabet or
        //! an underscore `_`, to make it valid variable name.
        while let Some(c) = self.peek() {
            // Checks if character is valid for a variable name
            if self.is_alpha(c) || c.is_ascii_digit() {
                self.advance();
            } else {
                // Break the loop if character couldn't be in valid variable name
                break;
            }
        }

        // Get the proper identifier type for current identifier and make a token for it
        self.make_token(self.identifier_type())
    }

    fn string(&mut self) -> Result<Token, ScannerError> {
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

    fn check_keyword(&self, start: usize, length: usize, rest: &str, ty: TokenType) -> TokenType {
        // When this function gets called, scanner is already at the of the lexeme and ready to
        // produce token.
        // Let's understand this with an example. Let's assume following things at the start of the
        // `scan_token()`
        // a. keyword = class, self.start = 21, self.current = 21
        // b. One character from lexeme gets consumed. self.current is 22 now.
        // c. `character` matches condition and `identifier()` is called. Function calls `advance()`
        //    4 times, and ends when it encounters a whitespace after class. self.current is 26 now
        // d. `identifier_type()` is being called, which compares first character of the current
        //    lexeme, to fixed characters. In this case, it matches with 'c'
        // e. `check_keyword()` is being called, with start=1 and length=4, and rest="lass"
        // f. self.current - self.start == start + length (26-21==1+4), condition is true
        // g. self.start(21) + start(1) = source_index_start = 22
        // i. source_index_start(22) + length(4) = source_index_end = 26
        // j. &self.source[22..26] = "lass"
        let source_index_start = self.start + start;
        let source_index_end = source_index_start + length;

        if self.current - self.start == start + length
            && &self.source[source_index_start..source_index_end] == rest
        {
            return ty;
        }

        TokenType::Identifier
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
