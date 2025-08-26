/// Represents all errors related to the scanner
#[derive(Debug, PartialEq)]
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