use crate::tokenize::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    counter: usize,
}

impl Lexer {
    pub fn new(contents: String) -> Self {
        Self {
            source: contents.chars().collect(),
            counter: 0,
        }
    }

    pub fn lex(&mut self) {
        let mut tokens: Vec<Token> = Vec::<Token>::new();

        while self.source.len() > self.counter {
            let c = self.current_char();
            match c {
                '=' => {
                    tokens.push(Token {
                        token_type: TokenType::Assign,
                        value: '='.to_string(),
                    });
                    self.counter += 1;
                }
                '\'' | '"' => {
                    self.counter += 1;

                    let mut buffer = String::new();




                    while self.current_char() != c {
                        buffer.push(self.current_char());

                        self.counter += 1;
                    }

                    tokens.push(Token::new(TokenType::Word, buffer));

                    self.counter += 1;
                }

                _ if c.is_alphabetic() => {
                    let mut buffer = String::new();
                    buffer.push(c);

                    self.counter += 1;


                    while self.current_char().is_alphabetic() {
                        buffer.push(self.current_char());
                        self.counter += 1;
                    }


                    let kind: TokenType = match buffer.as_str() {
                        "let" => TokenType::Let,
                        _ => TokenType::Identifier
                    };


                    tokens.push(Token::new(kind, buffer));


                }

                _ => self.counter += 1,
            }
        }


        println!("{:#?}", tokens);
    }

    fn current_char(&self) -> char {
        *self.source.get(self.counter).unwrap()
    }
}
