use std::{fmt::Debug, iter};

#[derive(Debug)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Self { token_type, value }
    }
}

#[derive(Debug)]
pub enum TokenType {
    Word,
    Symbol,
    Number(i64),
    LeftParen,
    RightParen,
    Plus,
    Dash,
    Star,
    EOF,
    Assign,
    Let,
    Identifier,
}

#[derive(Debug)]
pub struct SyntaxError {
    message: String,
}

impl SyntaxError {
    fn new(message: String) -> Self {
        SyntaxError { message }
    }
}

pub fn tokenizer(input: String) -> Result<Vec<TokenType>, SyntaxError> {
    let mut tokens: Vec<TokenType> = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(ch) = iter.next() {
        match ch {
            ch if ch.is_whitespace() => continue,
            ch if ch.is_alphabetic() => tokens.push(TokenType::Word),
            '(' => tokens.push(TokenType::LeftParen),
            ')' => tokens.push(TokenType::RightParen),
            '+' => tokens.push(TokenType::Plus),
            '-' => tokens.push(TokenType::Dash),
            '*' => tokens.push(TokenType::Star),
            '=' => tokens.push(TokenType::Assign), 
            '1'..='9' => {
                let n: i64 = iter::once(ch)
                    .chain(iter::from_fn(|| {
                        iter.by_ref().next_if(|s| s.is_ascii_digit())
                    }))
                    .collect::<String>()
                    .parse()
                    .unwrap();

                tokens.push(TokenType::Number(n));
            }
            _ => return Err(SyntaxError::new(format!("unrecognized character {}", ch))),
        }
    }

    tokens.push(TokenType::EOF);
    Ok(tokens)
}
