use crate::{compiler::{Compiler, CompilerError}, scanner::token::TokenType};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
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

pub type ParseFn<'a> = Option<fn(&mut Compiler<'a>)->Result<(), CompilerError>>;

#[derive(Debug, Clone, Copy)]
pub struct ParseRule<'a> {
    pub prefix: ParseFn<'a>,
    pub infix: ParseFn<'a>,
    pub precedence: Precedence,
}

impl<'a> ParseRule<'a> {
    fn get_rules() -> [ParseRule<'a>; 40] {
        [
            // TokenType::LeftParen
            ParseRule {
                prefix: Some(Compiler::grouping),
                infix: None,
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
                prefix: Some(Compiler::unary),
                infix: Some(Compiler::binary),
                precedence: Precedence::Term,
            },
            // TokenType::Plus
            ParseRule {
                prefix: None,
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
                infix: Some(Compiler::binary),
                precedence: Precedence::Factor,
            },
            // TokenType::Star
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Factor,
            },
            // TokenType::Bang
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::BangEqual
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
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
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Greater
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::GreatorEqual
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Less
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::LessEqual
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Identifier
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::String
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            // TokenType::Number
            ParseRule {
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
                prefix: None,
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
                prefix: None,
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
                prefix: None,
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

    pub fn get_parse_rule(ty: TokenType) -> ParseRule<'a> {
        let rules = Self::get_rules();
        rules[ty as usize]
    }
}