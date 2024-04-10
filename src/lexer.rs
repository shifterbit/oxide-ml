use std::collections::{hash_map, hash_set};
use std::fmt;
use std::fmt::Display;
use std::iter::Peekable;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    literal: String,
    position: Position,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\"{}\" {} {}",
            self.literal, self.token_type, self.position
        )
    }
}

impl Token {
    pub fn new(literal: String, token_type: TokenType, position: Position) -> Token {
        return Token {
            literal,
            token_type,
            position,
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    // Single Character Token
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Star,
    ForwardSlash,
    // Values
    Float(f64),
    Int(i64),
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LeftParen => write!(f, "LeftParen"),
            Self::RightParen => write!(f, "RightParen"),
            Self::Plus => write!(f, "Plus"),
            Self::Minus => write!(f, "Minus"),
            Self::Star => write!(f, "Star"),
            Self::ForwardSlash => write!(f, "ForwardSlash"),
            Self::Float(n) => write!(f, "Float({})", n),
            Self::Int(n) => write!(f, "Int({})", n),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    start_line: u32,
    end_line: u32,
    start_column: u32,
    end_column: u32,
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.start_line, self.start_column)
    }
}

impl Position {
    pub fn new(start_line: u32, end_line: u32, start_column: u32, end_column: u32) -> Position {
        return Position {
            start_line,
            end_line,
            start_column,
            end_column,
        };
    }
    pub fn new_inline(line: u32, start_column: u32, end_column: u32) -> Position {
        return Position::new(line, line, start_column, end_column);
    }
    pub fn new_single_char(line: u32, column: u32) -> Position {
        return Position::new(line, line, column, column);
    }
}

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(text: &str) -> Lexer {
        let mut tokens: Vec<Token> = Vec::new();
        let mut line = 1;
        let mut column = 1;
        let mut chars = text.chars().peekable();
        while let Some(character) = chars.peek() {
            match character {
                '\n' => {
                    line += 1;
                    column = 1;
                    chars.next();
                }
                ' ' => {
                    column += 1;
                    chars.next();
                }
                '\t' => {
                    column += 4;
                    chars.next();
                }
                '(' | ')' | '+' | '-' | '*' | '/' => {
                    let token = match_single_character_token(*character, line, column);
                    tokens.push(token.unwrap());
                    column += 1;
                    chars.next();
                }
                _c if is_digit(character) => {
                    let literal = read_number(&mut chars);
                    let length = literal.len() as u32;
                    let token = match_number(&literal, line, column);
                    tokens.push(token);
                    column += length;
                }
                _c if is_alpha(character) => {
                    column += 1;
                    chars.next();
                }
                _ => {
                    column += 1;
                    chars.next();
                }
            }
        }
        tokens.reverse();
        return Lexer { tokens };
    }
    pub fn peek(self: &Self) -> Option<&Token> {
        return self.tokens.last();
    }
    pub fn next(self: &mut Self) -> Option<&Token> {
        let _ = self.tokens.pop();
        return self.tokens.last();
    }

    pub fn tokens(self: Self) -> Vec<Token> {
        return self.tokens.clone();
    }

    
}

static LETTERS: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

static DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn is_alpha(character: &char) -> bool {
    return LETTERS.contains(character);
}

fn is_digit(character: &char) -> bool {
    return DIGITS.contains(character);
}

fn read_number<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> String {
    let mut num_str: String = "".to_string();
    let mut found_decimal = false;
    while let Some(c) = chars.peek() {
        match c {
            ch if is_digit(c) => {
                num_str.push(ch.to_owned());
                chars.next();
            }
            '.' if !found_decimal => {
                num_str.push('.');
                found_decimal = true;
                chars.next();
            }
            _ => break,
        }
    }
    return num_str;
}

fn match_number(num_str: &str, line: u32, column: u32) -> Token {
    let end_column: u32 = column + num_str.len() as u32;
    let position = Position::new_inline(line, column, end_column);
    if num_str.contains('.') {
        let float_val: f64 = num_str.parse().unwrap();
        return Token::new(num_str.to_string(), TokenType::Float(float_val), position);
    } else {
        let int_val: i64 = num_str.parse().unwrap();
        return Token::new(num_str.to_string(), TokenType::Int(int_val), position);
    }
}

#[derive(Debug)]
struct InvalidTokenError;
fn match_single_character_token(
    character: char,
    line: u32,
    column: u32,
) -> Result<Token, InvalidTokenError> {
    let position = Position::new_single_char(line, column);
    let token_type = match character {
        '(' => TokenType::LeftParen,
        ')' => TokenType::RightParen,
        '-' => TokenType::Minus,
        '+' => TokenType::Plus,
        '*' => TokenType::Star,
        '/' => TokenType::ForwardSlash,
        _ => return Err(InvalidTokenError),
    };
    let token = Token::new(character.to_string(), token_type, position);
    return Ok(token);
}
