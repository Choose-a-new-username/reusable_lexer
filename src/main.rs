use std::str::Chars;
use std::env;
use std::fs;

const EOF: char = '\0';

#[derive(Debug, Clone)]
enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
}

#[derive(Debug, Clone)]
enum TokenKind<'a> {
    Opr(Op),
    Ident(&'a str),
    Num(i32),
    OpeningBracket,
    ClosingBracket
}

#[derive(Debug, Clone)]
struct Token<'a> {
    kind: TokenKind<'a>,
    position: (usize, usize)
}

#[derive(Debug, Clone)]
struct Lexer<'a> {
    source: &'a str,
    prev: char,
    chars: Chars<'a>,
    pos: usize,
    row: usize,
    col: usize
}

#[allow(unused)]
impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        let mut chars = source.chars();
        Self {
            source,
            prev: chars.next().unwrap_or(EOF),
            chars,
            pos: 0,
            row: 1,
            col: 1
        }
    }

    fn pos(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    fn is_over(&self) -> bool {
        self.prev == EOF
    }

    #[inline]
    fn slice(&self, a: usize, b: usize) -> &'a str {
        &self.source[a..b]
    }

    #[inline]
    fn next_char(&mut self) -> Option<char> {
        if let Some(ch) = self.chars.next() {
            self.prev = ch;
            self.pos += ch.len_utf8();
            self.col += 1;
            if self.prev == '\n' {
                self.col = 0;
                self.row += 1;
            }
            Some(self.prev)
        } else {
            self.prev = EOF; // we're using EOF instead of Option::None for both performance and readabililty purposes.
            None
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn unicode_escape(&mut self, ch: char) -> Option<char> {
        // e.g. \n \t \u{...}
        let res = match ch {
            'n'   => '\n',
            't'   => '\t',
            'r'   => '\r',
            'u'   => todo!("implement unicode character codes"),
            'x'   => todo!("implement hex character codes"),
            other => other
        };
        Some(res)
    }

    fn trim_ident(&mut self) -> &'a str {
        let start_pos = self.pos;

        while self.prev.is_alphanumeric() || self.prev == '_' {
            self.next_char();
        }

        self.slice(start_pos, self.pos)
    }

    fn trim_number(&mut self) -> &'a str {
        let start_pos = self.pos;

        while self.prev.is_numeric() {
            self.next_char();
        }

        self.slice(start_pos, self.pos)
    }

    fn trim_comment(&mut self) {
        while self.prev != '\n' {
            self.next_char();
        }
    }

    fn trim_whitespace(&mut self) {
        while self.prev.is_whitespace() {
            self.next_char();
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        loop {
            self.trim_whitespace();
            let position = self.pos();

            let kind = match self.prev {
                'a'..='z' | 'A'..='Z' | '_' => Some(TokenKind::Ident(self.trim_ident())),
                '0'..='9' => Some(TokenKind::Num(self.trim_number().parse().unwrap_or(0))),
                '+' => {
                    self.next_char();
                    Some(TokenKind::Opr(Op::Plus))
                },
                '-' => {
                    self.next_char();
                    Some(TokenKind::Opr(Op::Minus))
                },
                '*' => {
                    self.next_char();
                    Some(TokenKind::Opr(Op::Multiply))
                },
                '/' => {
                    self.next_char();
                    if self.prev == '/' {
                        self.trim_comment();
                        continue;
                    } else {
                        Some(TokenKind::Opr(Op::Divide))
                    }
                },
                '%' => {
                    self.next_char();
                    Some(TokenKind::Opr(Op::Modulo))
                },
                '=' => {
                    self.next_char();
                    Some(TokenKind::Opr(Op::Equal))
                },
                '>' => {
                    self.next_char();
                    if self.prev == '=' {
                        self.next_char();
                        Some(TokenKind::Opr(Op::GreaterOrEqual))
                    } else {
                        Some(TokenKind::Opr(Op::Greater))
                    }
                },
                '<' => {
                    self.next_char();
                    if self.prev == '=' {
                        self.next_char();
                        Some(TokenKind::Opr(Op::LessOrEqual))
                    } else if self.prev == '>' {
                        self.next_char();
                        Some(TokenKind::Opr(Op::NotEqual))
                    } else {
                        Some(TokenKind::Opr(Op::Less))
                    }
                },
                '(' => {
                    self.next_char();
                    Some(TokenKind::OpeningBracket)
                },
                ')' => {
                    self.next_char();
                    Some(TokenKind::ClosingBracket)
                },
                _ => None
            };

            return if let Some(kind) = kind {
                Some(Token { kind, position })
            } else {
                None
            }
        }
    }
}

macro_rules! elapsed {
    ($($code:tt)*) => {
        {
            let start = std::time::Instant::now();
            $($code)*;
            println!("{:?}", start.elapsed());
        }
    }
}

fn main() {
    for (i, arg) in env::args().enumerate() {
        if i == 0 { continue }
        let file = fs::read_to_string(arg).expect("failed to read file");
        let mut lexer: Lexer<'_> = Lexer::new(&file);

        elapsed!(while let Some(tok) = lexer.next() {
            println!("{tok:?}");
        });
    }
}
