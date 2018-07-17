use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum Token {
    Number(f32),
    Plus,
    Minus,
    Star,
    Slash,
}

struct Lexer {
    source: String,
    position: usize,
}

impl Lexer {
    pub fn from(source: String) -> Lexer {
        Lexer {
            source: source,
            position: 0,
        }
    }

    fn skip_whitespace(&mut self) {
        self.position += self.source[self.position..]
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect::<String>()
            .len()
    }

    fn number(&self) -> String {
        let mut found_dot = false;
        self.source[self.position..]
            .chars()
            .take_while(|n| match n {
                n if n.is_numeric() => true,
                '.' if !found_dot => {
                    found_dot = true;
                    true
                }
                _ => false,
            })
            .collect::<String>()
    }

    fn next_char(&self) -> char {
        self.source[self.position..].chars().next().unwrap()
    }

    fn scan(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.position == self.source.len() {
            return None;
        }

        match self.next_char() {
            '0'...'9' => {
                let lexeme = self.number();
                self.position += lexeme.len();
                Some(Token::Number(lexeme.parse().unwrap()))
            }
            '+' => {
                self.position += '+'.len_utf8();
                Some(Token::Plus)
            }
            '-' => {
                self.position += '-'.len_utf8();
                Some(Token::Minus)
            }
            '*' => {
                self.position += '*'.len_utf8();
                Some(Token::Star)
            }
            '/' => {
                self.position += '/'.len_utf8();
                Some(Token::Slash)
            }
            _ => None,
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.scan()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match handle.read_line(&mut input) {
            Ok(_) => {
                let lexer = Lexer::from(input);
                for token in lexer {
                    println!("{:?}", token);
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}
