use crate::{
    compiler::{Compiler, CompilerError},
    scanner::token::TokenType,
};

/// `#[repr(u8)] means its memory layout will be equivalent to byte`
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
/// Order of `Precedence` variant matters. Because it will be converted to bytes and will be
/// incremented, order is important here.
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

/// Converts a byte to `enum Precedence`
impl From<u8> for Precedence {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Assignment,
            2 => Self::Or,
            3 => Self::And,
            4 => Self::Equality,
            5 => Self::Comparison,
            6 => Self::Term,
            7 => Self::Factor,
            8 => Self::Unary,
            9 => Self::Call,
            10 => Self::Primary,
            _ => unreachable!(),
        }
    }
}

/// This is type of pointer to the function, implemented in `Compiler` struct
pub type ParseFn<'a> = Option<fn(&mut Compiler<'a>, bool) -> Result<(), CompilerError>>;

#[derive(Debug, Clone, Copy)]
/// Data structure used to store infix and prefix rules of `TokenType`. Rules are just method
/// being executed dynamically if a specific `TokenType` has one.
/// Each `TokenType` has a separate `ParseRule`
pub struct ParseRule<'a> {
    pub prefix: ParseFn<'a>,
    pub infix: ParseFn<'a>,
    pub precedence: Precedence,
}

impl<'a> ParseRule<'a> {
    /// Gets all the rules for every token type
    /// We'll be accessing these rules by index, so order should be the same
    /// as the order of TokenType variants. We could assign numbers to each
    /// TokenType, but it looks tedious. It may change in future though.
    /// Another considerable Option is to use HashMap to store by TokenyType, which
    /// would be explored in future
    fn get_rules() -> [ParseRule<'a>; 40] {
        [
            // TokenType::LeftParen
            ParseRule {
                // This token token type is responsible to start executing grouping expressions.
                // It doesn't require another operand and should be at the start, we say that it's
                // a prefix rule
                prefix: Some(Compiler::grouping),
                infix: None,
                // Token itself shouldn't have any precedence. It's the inner expression which
                // has precedence
                precedence: Precedence::None,
            },
            // TokenType::RightParen
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::LeftBrace
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::RightBrace
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Comma
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Dot
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Minus
            ParseRule {
                // If it involves only one operand, it's a prefix and is unary operation
                prefix: Some(Compiler::unary),
                // If it involves two operands, it's infix and is a binary operation
                infix: Some(Compiler::binary),
                precedence: Precedence::Term,
            },
            // TokenType::Plus
            ParseRule {
                prefix: None,
                // Only a binary operation
                infix: Some(Compiler::binary),
                precedence: Precedence::Term,
            },
            // TokenType::Semicolon
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Slash
            ParseRule {
                prefix: None,
                // Only a binary operation
                infix: Some(Compiler::binary),
                precedence: Precedence::Factor,
            },
            // TokenType::Star
            ParseRule {
                prefix: None,
                // Only a binary operation
                infix: Some(Compiler::binary),
                precedence: Precedence::Factor,
            },
            // TokenType::Bang
            ParseRule {
                prefix: Some(Compiler::unary),
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::BangEqual
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Equality,
            },
            // TokenType::Equal
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::EqualEqual
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Equality,
            },
            // TokenType::Greater
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Comparison,
            },
            // TokenType::GreatorEqual
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Comparison,
            },
            // TokenType::Less
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Comparison,
            },
            // TokenType::LessEqual
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Comparison,
            },
            // TokenType::Identifier
            ParseRule {
                prefix: Some(Compiler::variable),
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::String
            ParseRule {
                prefix: Some(Compiler::string),
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Number
            ParseRule {
                // It means it's going to start parsing a number. Number itself doesn't
                // have any operator and operands, so it's going to be prefix rule.
                prefix: Some(Compiler::number),
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::And
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Class
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Else
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::False
            ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::For
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Fun
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::If
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Nil
            ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Or
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Print
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Return
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Super
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::This
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::True
            ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Var
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::While
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Error
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Eof
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        ]
    }

    /// Returns rule by type of token.
    pub fn get_parse_rule(ty: TokenType) -> ParseRule<'a> {
        let rules = Self::get_rules();
        // Since order of types in `TokenType` enum is same as rules specified for
        // the token type, it's safe to use type `ty` as index.
        rules[ty as usize]
    }
}
