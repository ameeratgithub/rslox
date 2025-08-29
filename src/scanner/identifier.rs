use crate::scanner::{
    Scanner,
    token::{Token, TokenType},
};

impl<'a> Scanner<'a> {
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
                        'o' => self.check_keyword(2, 1, "r", TokenType::For),
                        // Checks for keyword `fun`
                        'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
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
    pub(super) fn identifier(&mut self) -> Token {
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
}
