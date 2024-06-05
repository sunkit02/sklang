pub type Result<T> = core::result::Result<T, LexerError>;

#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnexpectedEof {
        line: usize,
        col: usize,
        expected: String,
    },
    UnexpectedCharacter {
        line: usize,
        col: usize,
        expected: String,
        got: char,
    },
    UnknownCharacter {
        line: usize,
        col: usize,
        character: char,
    },
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEof {
                line,
                col,
                expected: message,
            } => write!(
                f,
                "[line {line}: {col}] Unexpected end of file, expected {message}."
            ),
            Self::UnexpectedCharacter {
                line,
                col,
                expected,
                got,
            } => {
                write!(
                    f,
                    "[line {line}: {col}] Unexpected character '{got}', expected '{expected}'"
                )
            }
            Self::UnknownCharacter {
                line,
                col,
                character,
            } => {
                write!(f, "[line {line}: {col}] Unknown character '{character}'")
            }
        }
    }
}

impl std::error::Error for LexerError {}
