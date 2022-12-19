#![allow(dead_code)]
//! This module was designed to be reusable between programming language projects.
use std::str::Chars;

/// Mathematical operations (e.g. +, -, *, /)
#[derive(Debug, Clone)]
pub enum Op {
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

/// The different kinds of token
#[derive(Debug, Clone)]
pub enum TokenKind<'a> {
    Opr(Op),
    Ident(&'a str),
    Num(i32),
    OpeningBracket,
    ClosingBracket
}

/// A lexical token
#[derive(Debug, Clone)]
pub struct Token<'a> {
    /// The token's kind
    pub kind: TokenKind<'a>,
    /// The token's position in file
    pub position: (usize, usize)
}

/// The lexer iterator
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    /// The source string being read
    source: &'a str,
    prev: char,
    /// The previous character
    chars: Chars<'a>,
    /// The utf-8 position in file
    pos: usize,
    /// The row the lexer is on
    row: usize,
    /// The column the lexer is on
    col: usize
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from a `&str`
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.chars();
        Self {
            source,
            prev: chars.next().unwrap_or('\0'),
            chars,
            pos: 0,
            row: 1,
            col: 1
        }
    }

    /// The lexer's position in the file
    #[inline]
    pub fn pos(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    /// Is the lexer over?
    #[inline]
    pub fn is_over(&self) -> bool {
        self.prev == '\0'
    }

    /// Takes a slice of the source file
    #[inline]
    fn slice(&self, a: usize, b: usize) -> &'a str {
        &self.source[a..b]
    }

    /// Advances the iterator, returning the next character
    #[inline]
    pub fn next_char(&mut self) -> Option<char> {
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
            self.prev = '\0';
            None
        }
    }

    /// Peeks the next character in the iterator
    #[inline]
    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    /// Removes an identifier from the start of the source string
    fn trim_ident(&mut self) -> &'a str {
        let start_pos = self.pos;

        while self.prev.is_alphanumeric() || self.prev == '_' {
            self.next_char();
        }

        self.slice(start_pos, self.pos)
    }

    /// Removes a number literal from the start of the source string
    fn trim_number(&mut self) -> &'a str {
        let start_pos = self.pos;

        while self.prev.is_numeric() {
            self.next_char();
        }

        self.slice(start_pos, self.pos)
    }

    /// Removes a comment from the start of the source string
    fn trim_comment(&mut self) {
        while self.prev != '\n' {
            self.next_char();
        }
    }

    /// Trims whitespace from the start of the string
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
