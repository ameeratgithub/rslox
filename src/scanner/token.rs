/// Supported characters and literals by our language.
/// `TokenType` should be fixed, predictable and comparable to make implementation
/// and error handling easier
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

/// Metadata about a token. It doesn't store token itself, but has some properties
/// to locate that token in source string
#[derive(Clone, Debug)]
pub struct Token {
    /// Type of token being stored
    pub ty: TokenType,
    /// Location in source string, usually index of first character in source string
    pub start: usize,
    /// Length of the token. Like `class` keyword has 5 characters and it's the length of
    /// the token in source string
    pub length: usize,
    /// In which line of the source code the token appeares.
    pub line: i32,
}

impl Token {
    /// Returns the fresh instance of Token
    #[must_use]
    pub fn new(ty: TokenType, start: usize, length: usize, line: i32) -> Self {
        Self {
            ty,
            start,
            length,
            line,
        }
    }

    /// Returns string form of current token
    #[must_use]
    pub fn as_str(&self, source: &str) -> String {
        source[self.start..self.start + self.length].to_owned()
    }
}
