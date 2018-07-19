use std::io;

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
    line: usize,
}

impl Scanner {
    pub fn from(source: &str) -> Scanner {
        Scanner {
            source: String::from(source),
            position: 0,
            line: 1,
        }
    }

    fn skip_whitespace(&mut self) {
        let mut new_lines = 0;
        self.position += self.source[self.position..]
            .chars()
            .take_while(|c| match c {
                '\n' => {
                    new_lines += 1;
                    true
                },
                x => x.is_whitespace(),
            })
            .collect::<String>()
            .len();
        self.line += new_lines;
    }

    fn skip_comment(&mut self) {
        self.position += self.source[self.position..]
            .chars()
            .take_while(|n| n != &'\n')
            .collect::<String>()
            .len();
    }

    fn scan_number(&mut self) -> String {
        let mut found_dot = false;
        let lexeme = self.source[self.position..]
            .chars()
            .take_while(|n| match n {
                n if n.is_numeric() => true,
                '.' if !found_dot => {
                    found_dot = true;
                    true
                }
                _ => false,
            })
            .collect::<String>();
        self.position += lexeme.len();
        lexeme
    }

    fn scan_string(&mut self) -> Result<String, io::Error> {
        self.position += '"'.len_utf8();
        let lexeme = self.source[self.position..]
            .chars()
            .take_while(|n| n != &'"')
            .collect::<String>();
        self.position += lexeme.len();
        if self.next_char_eq('"') {
            self.position += '"'.len_utf8();
            Ok(lexeme)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unterminated string, line {}.", self.line),
            ))
        }
    }

    fn scan_identifier(&mut self) -> String {
        let lexeme = self.source[self.position..]
            .chars()
            .take_while(|n| n.is_alphanumeric() || n == &'_')
            .collect::<String>();
        self.position += lexeme.len();
        lexeme
    }

    fn match_keyword(&self, keyword: &str) -> Option<Token> {
        match keyword {
            "and" => Some(Token::And),
            "class" => Some(Token::Class),
            "else" => Some(Token::Else),
            "false" => Some(Token::False),
            "for" => Some(Token::For),
            "fun" => Some(Token::Fun),
            "if" => Some(Token::If),
            "nil" => Some(Token::Nil),
            "or" => Some(Token::Or),
            "print" => Some(Token::Print),
            "return" => Some(Token::Return),
            "super" => Some(Token::Super),
            "this" => Some(Token::This),
            "true" => Some(Token::True),
            "var" => Some(Token::Var),
            "while" => Some(Token::While),
            _ => None,
        }
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
            '!' => {
                self.position += '!'.len_utf8();
                if self.next_char_eq('=') {
                    self.position += '='.len_utf8();
                    Some(Token::BangEqual)
                } else {
                    Some(Token::Bang)
                }
            }
            '=' => {
                self.position += '='.len_utf8();
                if self.next_char_eq('=') {
                    self.position += '='.len_utf8();
                    Some(Token::EqualEqual)
                } else {
                    Some(Token::Equal)
                }
            }
            '<' => {
                self.position += '<'.len_utf8();
                if self.next_char_eq('=') {
                    self.position += '='.len_utf8();
                    Some(Token::LessEqual)
                } else {
                    Some(Token::Less)
                }
            }
            '>' => {
                self.position += '>'.len_utf8();
                if self.next_char_eq('=') {
                    self.position += '='.len_utf8();
                    Some(Token::GreaterEqual)
                } else {
                    Some(Token::Greater)
                }
            }
            '0'...'9' => {
                let lexeme = self.scan_number();
                Some(Token::Number(lexeme.parse().unwrap()))
            }
            '"' => match self.scan_string() {
                Ok(lexeme) => Some(Token::String(lexeme)),
                Err(error) => {
                    error!("{}", error);
                    self.scan()
                }
            },
            '/' => {
                self.position += '/'.len_utf8();
                if self.next_char_eq('/') {
                    self.position += '/'.len_utf8();
                    self.skip_comment();
                    self.scan()
                } else {
                    Some(Token::Slash)
                }
            }
            c if c.is_alphabetic() => {
                let lexeme = self.scan_identifier();
                match self.match_keyword(&lexeme[..]) {
                    None => Some(Token::Identifier(lexeme)),
                    keyword => keyword,
                }
            }
            c => {
                error!("Unexpected character: {}, line {}.", c, self.line);
                self.position += c.len_utf8();
                self.scan()
            }
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
    fn skip_comment_without_text() {
        let mut scanner = Scanner::from("//");
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn skip_comment_with_text() {
        let mut scanner = Scanner::from("// skip me");
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn skip_comment_followed_by_plus() {
        let mut scanner = Scanner::from("// skip me\n+");
        assert_eq!(scanner.next(), Some(Token::Plus));
    }

    #[test]
    fn scan_string() {
        let mut scanner = Scanner::from("\"Hello 🌎!\"");
        assert_eq!(
            scanner.next(),
            Some(Token::String(String::from("Hello 🌎!")))
        );
    }

    #[test]
    fn scan_string_and_number() {
        let mut scanner = Scanner::from("\"Hello 🌎!\" 42");
        assert_eq!(
            scanner.next(),
            Some(Token::String(String::from("Hello 🌎!")))
        );
        assert_eq!(scanner.next(), Some(Token::Number(42.0)));
    }

    #[test]
    fn scan_unterminated_string() {
        let mut scanner = Scanner::from("\"Hello 🌎!");
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn scan_identfier() {
        let mut scanner = Scanner::from("hello_world");
        assert_eq!(
            scanner.next(),
            Some(Token::Identifier(String::from("hello_world")))
        );
    }

    #[test]
    fn scan_class_keyword() {
        let mut scanner = Scanner::from("class");
        assert_eq!(scanner.next(), Some(Token::Class));
    }

    #[test]
    fn scan_and_keyword() {
        let mut scanner = Scanner::from("and");
        assert_eq!(scanner.next(), Some(Token::And));
    }

    #[test]
    fn scan_for_loop() {
        let mut scanner = Scanner::from(
            "for (var a = 1; a < 10; a = a + 1) {
  print a;
}",
        );
        assert_eq!(scanner.next(), Some(Token::For));
        assert_eq!(scanner.next(), Some(Token::LeftParen));
        assert_eq!(scanner.next(), Some(Token::Var));
        assert_eq!(scanner.next(), Some(Token::Identifier(String::from("a"))));
        assert_eq!(scanner.next(), Some(Token::Equal));
        assert_eq!(scanner.next(), Some(Token::Number(1.0)));
        assert_eq!(scanner.next(), Some(Token::Semicolon));
        assert_eq!(scanner.next(), Some(Token::Identifier(String::from("a"))));
        assert_eq!(scanner.next(), Some(Token::Less));
        assert_eq!(scanner.next(), Some(Token::Number(10.0)));
        assert_eq!(scanner.next(), Some(Token::Semicolon));
        assert_eq!(scanner.next(), Some(Token::Identifier(String::from("a"))));
        assert_eq!(scanner.next(), Some(Token::Equal));
        assert_eq!(scanner.next(), Some(Token::Identifier(String::from("a"))));
        assert_eq!(scanner.next(), Some(Token::Plus));
        assert_eq!(scanner.next(), Some(Token::Number(1.0)));
        assert_eq!(scanner.next(), Some(Token::RightParen));
        assert_eq!(scanner.next(), Some(Token::LeftBrace));
        assert_eq!(scanner.next(), Some(Token::Print));
        assert_eq!(scanner.next(), Some(Token::Identifier(String::from("a"))));
        assert_eq!(scanner.next(), Some(Token::Semicolon));
        assert_eq!(scanner.next(), Some(Token::RightBrace));
    }

    #[test]
    fn scan_unexpected() {
        let mut scanner = Scanner::from("💩");
        assert_eq!(scanner.next(), None);
    }
}
