#[derive(Debug, PartialEq)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Slash,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier(String),
    String(String),
    Number(f32),
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

pub struct Scanner {
    source: String,
    position: usize,
}

impl Scanner {
    pub fn from(source: &str) -> Scanner {
        Scanner {
            source: String::from(source),
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

    fn next_char(&self) -> Option<char> {
        self.source[self.position..].chars().next()
    }

    fn next_char_eq(&mut self, expected: char) -> bool {
        match self.next_char() {
            Some(c) if c == expected => true,
            Some(_) | None => false,
        }
    }

    fn scan(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.position == self.source.len() {
            return None;
        }

        match self.next_char().unwrap() {
            '(' => {
                self.position += '('.len_utf8();
                Some(Token::LeftParen)
            }
            ')' => {
                self.position += ')'.len_utf8();
                Some(Token::RightParen)
            }
            '{' => {
                self.position += '{'.len_utf8();
                Some(Token::LeftBrace)
            }
            '}' => {
                self.position += '}'.len_utf8();
                Some(Token::RightBrace)
            }
            ',' => {
                self.position += ','.len_utf8();
                Some(Token::Comma)
            }
            '.' => {
                self.position += '.'.len_utf8();
                Some(Token::Dot)
            }
            ';' => {
                self.position += ';'.len_utf8();
                Some(Token::Semicolon)
            }
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
            },
            '!' => {
                self.position += '!'.len_utf8();
                if self.next_char_eq('=') {
                    self.position += '='.len_utf8();
                    Some(Token::BangEqual)
                } else {
                    Some(Token::Bang)
                }
            },
            '=' => {
                self.position += '='.len_utf8();
                if self.next_char_eq('=') {
                    self.position += '='.len_utf8();
                    Some(Token::EqualEqual)
                } else {
                    Some(Token::Equal)
                }
            },
            '<' => {
                self.position += '<'.len_utf8();
                if self.next_char_eq('=') {
                    self.position += '='.len_utf8();
                    Some(Token::LessEqual)
                } else {
                    Some(Token::Less)
                }
            },
            '>' => {
                self.position += '>'.len_utf8();
                if self.next_char_eq('=') {
                    self.position += '='.len_utf8();
                    Some(Token::GreaterEqual)
                } else {
                    Some(Token::Greater)
                }
            },
            c => panic!("Unexpected character: {}.", c),
        }
    }
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.scan()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_left_paren() {
        let mut scanner = Scanner::from("(");
        assert_eq!(scanner.next(), Some(Token::LeftParen));
    }

    #[test]
    fn scan_right_paren() {
        let mut scanner = Scanner::from(")");
        assert_eq!(scanner.next(), Some(Token::RightParen));
    }

    #[test]
    fn scan_left_brace() {
        let mut scanner = Scanner::from("{");
        assert_eq!(scanner.next(), Some(Token::LeftBrace));
    }

    #[test]
    fn scan_right_brace() {
        let mut scanner = Scanner::from("}");
        assert_eq!(scanner.next(), Some(Token::RightBrace));
    }

    #[test]
    fn scan_comma() {
        let mut scanner = Scanner::from(",");
        assert_eq!(scanner.next(), Some(Token::Comma));
    }

    #[test]
    fn scan_dot() {
        let mut scanner = Scanner::from(".");
        assert_eq!(scanner.next(), Some(Token::Dot));
    }

    #[test]
    fn scan_minus() {
        let mut scanner = Scanner::from("-");
        assert_eq!(scanner.next(), Some(Token::Minus));
    }

    #[test]
    fn scan_plus() {
        let mut scanner = Scanner::from("+");
        assert_eq!(scanner.next(), Some(Token::Plus));
    }

    #[test]
    fn scan_semicolon() {
        let mut scanner = Scanner::from(";");
        assert_eq!(scanner.next(), Some(Token::Semicolon));
    }

    #[test]
    fn scan_star() {
        let mut scanner = Scanner::from("*");
        assert_eq!(scanner.next(), Some(Token::Star));
    }

    #[test]
    fn scan_slash() {
        let mut scanner = Scanner::from("/");
        assert_eq!(scanner.next(), Some(Token::Slash));
    }

    #[test]
    fn scan_number() {
        let mut scanner = Scanner::from("42");
        assert_eq!(scanner.next(), Some(Token::Number(42.0)));
    }

    #[test]
    fn scan_bang() {
        let mut scanner = Scanner::from("!");
        assert_eq!(scanner.next(), Some(Token::Bang));
    }

    #[test]
    fn scan_bang_equal() {
        let mut scanner = Scanner::from("!=");
        assert_eq!(scanner.next(), Some(Token::BangEqual));
    }

    #[test]
    fn scan_equal() {
        let mut scanner = Scanner::from("=");
        assert_eq!(scanner.next(), Some(Token::Equal));
    }

    #[test]
    fn scan_equal_equal() {
        let mut scanner = Scanner::from("==");
        assert_eq!(scanner.next(), Some(Token::EqualEqual));
    }

    #[test]
    fn scan_less() {
        let mut scanner = Scanner::from("<");
        assert_eq!(scanner.next(), Some(Token::Less));
    }

    #[test]
    fn scan_less_equal() {
        let mut scanner = Scanner::from("<=");
        assert_eq!(scanner.next(), Some(Token::LessEqual));
    }

    #[test]
    fn scan_greater() {
        let mut scanner = Scanner::from(">");
        assert_eq!(scanner.next(), Some(Token::Greater));
    }

    #[test]
    fn scan_greater_equal() {
        let mut scanner = Scanner::from(">=");
        assert_eq!(scanner.next(), Some(Token::GreaterEqual));
    }

    #[test]
    #[should_panic]
    fn scan_unexpected() {
        let mut scanner = Scanner::from("💩");
        scanner.next();
    }
}
