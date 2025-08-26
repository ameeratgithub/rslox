use crate::scanner::{Scanner, errors::ScannerError, token::TokenType};

#[test]
fn skip_whitespace() {
    let source = "   
               // This comment should also be ignored
               ";
    let mut scanner = Scanner::new(source);
    let mut token = scanner.scan_token().unwrap();
    while !scanner.is_at_end() {
        token = scanner.scan_token().unwrap();
    }

    assert_eq!(scanner.current, source.len());
    assert_eq!(token.ty, TokenType::Eof);
}

#[test]
fn single_character_tokens() {
    let source = "(){};,.-+/*! = ><";
    let token_tys = [
        TokenType::LeftParen,
        TokenType::RightParen,
        TokenType::LeftBrace,
        TokenType::RightBrace,
        TokenType::Semicolon,
        TokenType::Comma,
        TokenType::Dot,
        TokenType::Minus,
        TokenType::Plus,
        TokenType::Slash,
        TokenType::Star,
        TokenType::Bang,
        TokenType::Equal,
        TokenType::Greater,
        TokenType::Less,
        TokenType::Eof,
    ];
    let mut scanner = Scanner::new(source);
    let mut index = 0;
    while !scanner.is_at_end() {
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.ty, token_tys[index]);
        index += 1;
    }
}

#[test]
fn double_character_tokens() {
    let source = "!===>=<=";
    let token_tys = [
        TokenType::BangEqual,
        TokenType::EqualEqual,
        TokenType::GreaterEqual,
        TokenType::LessEqual,
        TokenType::Eof,
    ];
    let mut scanner = Scanner::new(source);
    let mut index = 0;
    while !scanner.is_at_end() {
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.ty, token_tys[index]);
        index += 1;
    }
}

#[test]
fn number_tokens() {
    let source = "1 1.23 0.00 123.1923 0.123";
    let mut scanner = Scanner::new(source);
    let mut total_items = 0;
    while !scanner.is_at_end() {
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.ty, TokenType::Number);
        total_items += 1;
    }
    assert!(total_items == 5);
}

#[test]
fn invalid_number_tokens() {
    let source = "-1 -121.23 123";
    let token_tys = [
        TokenType::Minus,
        TokenType::Number,
        TokenType::Minus,
        TokenType::Number,
        TokenType::Number,
        TokenType::Eof,
    ];

    let mut scanner = Scanner::new(source);
    let mut index = 0;
    while !scanner.is_at_end() {
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.ty, token_tys[index]);
        index += 1;
    }
    assert!(index == 5);
}

#[test]
fn string_tokens() {
    let source = "\"My\" \"name\" \"is\" \"Ameer\" \"Hamza\"";
    let mut scanner = Scanner::new(source);
    let mut total_items = 0;
    while !scanner.is_at_end() {
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.ty, TokenType::String);
        total_items += 1;
    }
    assert!(total_items == 5);
}

#[test]
fn invalid_string_token() {
    let source = "\"This is unterminated string";
    let mut scanner = Scanner::new(source);
    let result = scanner.scan_token();
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        ScannerError::UnterminatedString { line: 1 }
    );
}

#[test]
fn identifiers() {
    let source = "and or class if else false for fun 
    true nil print return super this _this _class another_variable_name";

    let token_tys = [
        TokenType::And,
        TokenType::Or,
        TokenType::Class,
        TokenType::If,
        TokenType::Else,
        TokenType::False,
        TokenType::For,
        TokenType::Fun,
        TokenType::True,
        TokenType::Nil,
        TokenType::Print,
        TokenType::Return,
        TokenType::Super,
        TokenType::This,
        TokenType::Identifier,
        TokenType::Identifier,
        TokenType::Identifier,
        TokenType::Eof,
    ];
    let mut scanner = Scanner::new(source);
    let mut index = 0;
    while !scanner.is_at_end() {
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.ty, token_tys[index]);
        index += 1;
    }
}
