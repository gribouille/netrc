use std::collections::VecDeque;
use std::str::Chars;

pub struct Lex<'a> {
    pub lineno: u32,
    pub instream: Chars<'a>,
    pub pushback: VecDeque<String>,
}

impl<'a> Lex<'a> {
    pub fn new(content: &'a str) -> Self {
        Lex {
            lineno: 1,
            instream: content.chars(),
            pushback: VecDeque::new(),
        }
    }

    pub fn read_char(&mut self) -> Option<char> {
        let ch = self.instream.next();
        if ch == Some('\n') {
            self.lineno += 1;
        }
        ch
    }

    pub fn read_line(&mut self) -> String {
        let mut s = String::new();
        for ch in &mut self.instream {
            if ch == '\n' {
                return s;
            }
            s.push(ch);
        }
        s
    }

    pub fn get_token(&mut self) -> String {
        let p = self.pushback.pop_front();
        if let Some(x) = p {
            return x;
        }
        let mut token = String::new();

        while let Some(ch) = self.read_char() {
            match ch {
                '\n' | '\t' | '\r' | ' ' => {
                    continue;
                }
                '"' => {
                    while let Some(ch) = self.read_char() {
                        match ch {
                            '"' => {
                                return token;
                            }
                            '\\' => {
                                token.push(self.read_char().unwrap_or(' '));
                            }
                            _ => {
                                token.push(ch);
                            }
                        }
                    }
                }
                _ => {
                    let c = if ch == '\\' {
                        self.read_char().unwrap_or(' ')
                    } else {
                        ch
                    };
                    token.push(c);
                    while let Some(ch) = self.read_char() {
                        let c = match ch {
                            '\n' | '\t' | '\r' | ' ' => {
                                return token;
                            }
                            '\\' => self.read_char().unwrap_or(' '),
                            _ => ch,
                        };
                        token.push(c);
                    }
                }
            }
        }
        token
    }

    pub fn push_token(&mut self, token: &str) {
        self.pushback.push_back(token.to_owned());
    }
}
