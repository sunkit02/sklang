use std::collections::HashMap;

use error::Result;
use lazy_static::lazy_static;

use crate::lexer::cursor::Peekable;

use self::{cursor::Cursor, error::LexerError};

pub mod cursor;
pub mod error;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("break", TokenType::Break);
        map.insert("continue", TokenType::Continue);
        map.insert("default", TokenType::Default);
        map.insert("else", TokenType::Else);
        map.insert("enum", TokenType::Enum);
        map.insert("false", TokenType::False);
        map.insert("fn", TokenType::Fn);
        map.insert("for", TokenType::For);
        map.insert("if", TokenType::If);
        map.insert("match", TokenType::Match);
        map.insert("return", TokenType::Return);
        map.insert("struct", TokenType::Struct);
        map.insert("switch", TokenType::Switch);
        map.insert("true", TokenType::True);
        map.insert("var", TokenType::Var);
        map.insert("while", TokenType::While);

        // Types
        map.insert("int", TokenType::Primitive(PrimitiveType::Int));
        map.insert("uint", TokenType::Primitive(PrimitiveType::UInt));
        map.insert("float", TokenType::Primitive(PrimitiveType::Float));
        map.insert("bool", TokenType::Primitive(PrimitiveType::Bool));
        map.insert("char", TokenType::Primitive(PrimitiveType::Char));
        map
    };
}

pub struct Lexer {
    source: Cursor,
    start: usize,
    current: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: Cursor::new(source),
            start: 0,
            current: 0,
            line: 1,
            col: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Result<Token>> {
        let mut results = Vec::new();

        loop {
            let token = self.next_token();
            if let Ok(ref token) = token {
                if token.ttype == TokenType::Eof {
                    break;
                }
            }
            results.push(token);
        }

        results
    }

    #[inline]
    fn next_token(&mut self) -> Result<Token> {
        let Some(ch) = self.advance() else {
            return Ok(Token {
                ttype: TokenType::Eof,
                line: self.line,
                col: self.col,
            });
        };

        let ttype = match ch {
            '+' => match self.source.peek_nth(0) {
                Some(next) if next == '=' => {
                    self.advance();
                    TokenType::AddEqual
                }
                _ => TokenType::Add,
            },
            '-' => match self.source.peek_nth(0) {
                Some(next) if next == '=' => {
                    self.advance();
                    TokenType::LessEqual
                }
                Some(next) if next == '>' => {
                    self.advance();
                    TokenType::Arrow
                }
                _ => TokenType::Less,
            },
            '*' => match self.source.peek_nth(0) {
                Some(next) if next == '=' => {
                    self.advance();
                    TokenType::StarEqual
                }
                _ => TokenType::Star,
            },
            '/' => match self.source.peek_nth(0) {
                Some(next) if next == '=' => {
                    self.advance();
                    TokenType::SlashEqual
                }
                Some(next) if next == '/' => {
                    self.handle_comment();
                    TokenType::Comment
                }
                _ => TokenType::Slash,
            },
            '%' => match self.source.peek_nth(0) {
                Some(next) if next == '=' => {
                    self.advance();
                    TokenType::ModuloEqual
                }
                _ => TokenType::Modulo,
            },
            '!' => match self.source.peek_nth(0) {
                Some(next) if next == '=' => {
                    self.advance();
                    TokenType::BangEqual
                }
                _ => TokenType::Bang,
            },
            '=' => match self.source.peek_nth(0) {
                Some(next) if next == '=' => {
                    self.advance();
                    TokenType::EqualEqual
                }
                _ => TokenType::Equal,
            },
            '>' => match self.source.peek_nth(0) {
                Some(next) if next == '=' => {
                    self.advance();
                    TokenType::GreaterEqual
                }
                _ => TokenType::Greater,
            },
            '<' => match self.source.peek_nth(0) {
                Some(next) if next == '=' => {
                    self.advance();
                    TokenType::LessEqual
                }
                _ => TokenType::Less,
            },
            '&' => match self.source.peek_nth(0) {
                Some(next) if next == '&' => {
                    self.advance();
                    TokenType::LogicalAnd
                }
                _ => TokenType::Ampersand,
            },
            '|' => match self.source.peek_nth(0) {
                Some(next) if next == '|' => {
                    self.advance();
                    TokenType::LogicalOr
                }
                _ => TokenType::Bar,
            },
            ':' => TokenType::Colon,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '{' => TokenType::LeftBrace,
            '[' => TokenType::LeftBracket,
            '(' => TokenType::LeftParen,
            '}' => TokenType::RightBrace,
            ']' => TokenType::RightBracket,
            ')' => TokenType::RightParen,
            ';' => TokenType::Semicolon,

            '\'' => self.handle_char()?,
            '"' => self.handle_string()?,
            ch if ch.is_numeric() => self.handle_number()?,
            ch if ch.is_alphanumeric() || ch == '_' => self.handle_identifier()?,

            '\n' => {
                self.line += 1;
                self.col = 1;
                return self.next_token();
            }

            ch if ch.is_whitespace() => {
                self.start = self.current;
                return self.next_token();
            }
            _ => {
                return Err(LexerError::UnknownCharacter {
                    line: self.line,
                    col: self.col,
                    character: ch,
                })
            }
        };

        self.start = self.current;

        Ok(Token {
            ttype,
            line: self.line,
            col: self.col,
        })
    }

    fn handle_comment(&mut self) {
        _ = self.source.next().expect("second slash in comment start");

        while let Some(ch) = self.source.peek_nth(1) {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn handle_char(&mut self) -> Result<TokenType> {
        let ch = self.advance().ok_or_else(|| LexerError::UnexpectedEof {
            line: self.line,
            col: self.col,
            expected: "a character".to_owned(),
        })?;

        self.consume('\'')?;

        Ok(TokenType::Character(ch))
    }

    fn handle_string(&mut self) -> Result<TokenType> {
        while let Some(ch) = self.source.peek_nth(1) {
            if ch == '\n' {
                return Err(LexerError::UnexpectedCharacter {
                    line: self.line,
                    col: self.col,
                    expected: "a valid string".to_owned(),
                    got: ch,
                });
            }

            if ch == '"' {
                break;
            }

            // TODO: Implement escape sequences

            self.source.next();
        }

        let string = self
            .source
            .substring(self.start, self.current)
            .expect("start and current should be valid");

        Ok(TokenType::String(string))
    }

    fn handle_number(&mut self) -> Result<TokenType> {
        let mut is_float = false;
        while let Some(ch) = self.source.peek_nth(0) {
            match ch {
                '0'..='9' => {
                    self.advance();
                }
                '.' => {
                    is_float = true;
                    self.advance();
                }
                _ => break,
            }
        }

        let lexeme = self.get_lexeme();
        let msg = "parsing should never fail";

        if is_float {
            Ok(TokenType::Decimal(lexeme.parse::<f64>().expect(msg)))
        } else {
            Ok(TokenType::Integer(lexeme.parse::<u64>().expect(msg)))
        }
    }

    fn handle_identifier(&mut self) -> Result<TokenType> {
        while let Some(ch) = self.source.peek_nth(0) {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme = self.get_lexeme();

        Ok(KEYWORDS
            .get(lexeme.as_str())
            .cloned()
            .unwrap_or(TokenType::Identifier(lexeme)))
    }

    fn consume(&mut self, target: char) -> Result<char> {
        let next = self
            .source
            .peek_nth(1)
            .ok_or_else(|| LexerError::UnexpectedEof {
                line: self.line,
                col: self.col,
                expected: format!("{target}"),
            })?;

        if next != target {
            return Err(LexerError::UnexpectedCharacter {
                line: self.line,
                col: self.col,
                expected: target.to_string(),
                got: next,
            });
        }

        Ok(self.advance().expect("next char should exist"))
    }

    fn advance(&mut self) -> Option<char> {
        let next = self.source.next();
        next.inspect(|ch| match ch {
            '\n' => {
                self.col = 1;
                self.line += 1;
                self.current = 0;
                self.start = 0;
            }
            _ => {
                self.col += 1;
                self.current += 1;
            }
        })
    }

    fn get_lexeme(&self) -> String {
        self.source
            .substring(self.start, self.current)
            .expect("start and current should always be valid")
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    ttype: TokenType,
    line: usize,
    col: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{} {:?})", self.line, self.col, self.ttype)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Add,
    AddEqual,
    Minus,
    MinusEqual,
    Modulo,
    ModuloEqual,
    Slash,
    SlashEqual,
    Star,
    StarEqual,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Ampersand,
    /// ->
    Arrow,
    /// |
    Bar,
    Colon,
    Comma,
    Dot,
    LeftBrace,
    LeftBracket,
    LeftParen,
    LogicalAnd,
    LogicalOr,
    RightBrace,
    RightBracket,
    RightParen,
    Semicolon,

    // Literals
    Character(char),
    Decimal(f64),
    Identifier(String),
    Integer(u64),
    String(String),

    Primitive(PrimitiveType),

    // Keywords
    Break,
    Continue,
    Default,
    Else,
    Enum,
    False,
    Fn,
    For,
    If,
    Match,
    Return,
    Struct,
    Switch,
    True,
    Var,
    While,

    Comment,
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrimitiveType {
    // Types
    Int,
    UInt,
    Float,
    Bool,
    Char,
}
