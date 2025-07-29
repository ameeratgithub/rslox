use crate::scanner::{Scanner, ScannerError};

// todo! implement display trait
#[derive(Debug)]
pub enum CompilerError {
    ScannerError(ScannerError),
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
