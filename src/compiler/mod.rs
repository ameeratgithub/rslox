use crate::scanner::{Scanner, ScannerError};

#[derive(Debug)]
pub enum CompilerError {
    ScannerError(ScannerError),
}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ScannerError(error) => {
                write!(f, "{}", error)
            }
        }
    }
}

pub fn compile(source: &str) -> Result<(), CompilerError> {
    let mut scanner = Scanner::new(source);
    let mut line = -1;

    loop {
        let token = scanner
            .scan_token()
            .map_err(|e| CompilerError::ScannerError(e))?;

        if token.line != line {
            print!("{:4} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        let token_ty = token.ty as u8;
        let token_str = &source[token.start..token.start + token.length as usize];
        println!("{:2} '{}'", token_ty, token_str);

        if scanner.is_at_end() {
            return Ok(());
        }
    }
}
