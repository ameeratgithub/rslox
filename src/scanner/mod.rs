use crate::scanner::token::{Token, TokenType};

pub mod token;

#[derive(Debug)]
pub enum ScannerError {
    UnexpectedCharacter { line: i32, character: char },
    UnterminatedString { line: i32 },
}

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

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn number(&mut self) -> Token {
        while let Some(c) = self.peek()
            && c.is_ascii_digit()
        {
            self.advance();
        }

        if let Some(c) = self.peek()
            && c == '.'
            && let Some(ch) = self.peek_next()
            && ch.is_ascii_digit()
        {
            self.advance();

            while let Some(c) = self.peek()
                && c.is_ascii_digit()
            {
                self.advance();
            }
        }

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

    fn skip_whitespace(&mut self) {
        loop {
            if let Some(c) = self.peek() {
                match c {
                    ' ' | '\r' | '\t' => {
                        self.advance();
                        break;
                    }
                    '\n' => {
                        self.line += 1;
                        self.advance();
                        break;
                    }
                    '/' => {
                        // We need `peek_next()` because we're just looking at current character, and
                        // not advancing, looking at current character and looking to match next
                        // character would require `peek_next()`
                        if let Some(c) = self.peek_next()
                            && c == '/'
                        {
                            while let Some(c) = self.peek()
                                && c != '\n'
                                && !self.is_at_end()
                            {
                                self.advance();
                            }
                        } else {
                            return;
                        }

                        break;
                    }
                    _ => return,
                }
            }
        }
    }

    fn identifier_type(&self) -> TokenType {
        // Since we've already consumed at least one character, it's safe to unwrap here
        // as `self.start` is the starting index of token
        let starting_char = self.source[self.start..].chars().next().unwrap();
        match starting_char {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'f' => {
                // It means more than 1 characters have been processed
                if self.current - self.start > 1 {
                    // Since more than 1 characters have been processed, it's save to unwrap second
                    // character.
                    let second_char = self.source[self.start + 1..].chars().next().unwrap();
                    // Keywords starting with 'f' can have one of 'a', 'o', 'u' as second character
                    // so, we'll try to match with pre-defined keywords.
                    match second_char {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                        'o' => self.check_keyword(2, 1, "r", TokenType::False),
                        'u' => self.check_keyword(2, 1, "n", TokenType::False),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            't' => {
                if self.current - self.start > 1 {
                    // Since more than 1 characters have been processed, it's save to unwrap second
                    // character.
                    let second_char = self.source[self.start + 1..].chars().next().unwrap();
                    // Keywords starting with 't' can have one of 'h', 'r' as second character
                    // so, we'll try to match with pre-defined keywords.
                    match second_char {
                        'h' => self.check_keyword(2, 2, "is", TokenType::This),
                        'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            _ => TokenType::Identifier,
        }
    }

    fn identifier(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if self.is_alpha(c) || c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

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
            if let Some(c) = self.peek()
                && c == '\n'
            {
                self.line += 1;
            }
            self.advance();
        }

        // Remember we didn't consume closing '"' of a string. If scanner's already at the end
        // then it's unterminated string.
        if self.is_at_end() {
            return Err(ScannerError::UnterminatedString { line: self.line });
        }

        // Consume closing '"'
        self.advance();
        Ok(self.make_token(TokenType::String))
    }

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

    fn advance(&mut self) -> Option<char> {
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

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let Some(c) = self.peek() {
            if c != expected {
                return false;
            }
            // If matches, consume character to make token
            self.current += 1;
            return true;
        }

        false
    }

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

    fn make_token(&self, ty: TokenType) -> Token {
        Token::new(
            ty,
            self.start,
            (self.current - self.start) as u32,
            self.line,
        )
    }
}
