use ac_ir::syntax::{Base, LiteralKind};
use unicode_xid::UnicodeXID;

use crate::lexer::cursor::{Cursor, Token, TokenKind};

use LiteralKind::*;
use TokenKind::*;

impl<'src> Cursor<'src> {
    /// Creates an iterator that produces tokens from the input string.
    pub fn tokenize(input: &'src str) -> impl Iterator<Item = Token> {
        let mut cursor = Self::new(input);
        std::iter::from_fn(move || {
            let token = cursor.advance_token();
            if token.kind != TokenKind::EOF {
                Some(token)
            } else {
                None
            }
        })
    }

    /// Parses a token from the input string.
    pub fn advance_token(&mut self) -> Token {
        let Some(first_char) = self.bump() else {
            return Token { kind: EOF, len: 0 };
        };

        let kind = match first_char {
            '/' => match self.first() {
                '/' => self.line_comment(),
                '*' => self.block_comment(),
                _ => Slash,
            },

            '\u{000D}' => match self.first() {
                '\u{000A}' => {
                    self.bump();

                    NL
                }
                _ => self.ws(),
            },
            '\u{000A}' => NL,

            c if is_whitespace(c) => self.ws(),

            '0'..='9' => Literal {
                kind: self.number(first_char),
            },

            '`' => self.raw_ident(),

            c if is_ident_start(c) => self.ident(),

            '&' => And,
            ',' => Comma,
            ':' => Colon,
            '.' => Dot,
            '=' => Eq,
            '!' => Excl,
            '>' => GT,
            '<' => LT,
            '-' => Minus,
            '|' => Or,
            '%' => Percent,
            '+' => Plus,
            '?' => Quest,
            ';' => Semi,
            '*' => Star,
            '~' => Tilde,

            '{' => LBrace,
            '}' => RBrace,
            '(' => LParen,
            ')' => RParen,

            _ => Unknown,
        };

        let len = self.bumped_len();
        self.reset_len_remaining();

        Token { kind, len }
    }

    fn block_comment(&mut self) -> TokenKind {
        self.bump();

        let mut depth = 1usize;
        while let Some(c) = self.bump() {
            match c {
                '/' if self.first() == '*' => {
                    self.bump();
                    depth += 1;
                }
                '*' if self.first() == '/' => {
                    self.bump();
                    depth -= 1;
                    if depth == 0 {
                        // This block comment is closed, so for a construction like "/* */ */"
                        // there will be a successfully parsed block comment "/* */"
                        // and " */" will be processed separately.
                        break;
                    }
                }
                _ => (),
            }
        }

        BlockComment {
            terminated: depth == 0,
        }
    }

    fn line_comment(&mut self) -> TokenKind {
        self.eat_until(b'\n');

        LineComment
    }

    fn ws(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);

        WS
    }

    fn number(&mut self, first_digit: char) -> LiteralKind {
        use Base::*;
        let mut base = Dec;
        if first_digit == '0' {
            match self.first() {
                'b' | 'B' => {
                    base = Bin;
                    self.bump();
                    if !self.eat_dec_digits() {
                        return Int {
                            base,
                            empty_int: true,
                        };
                    }
                }
                'o' | 'O' => {
                    base = Oct;
                    self.bump();
                    if !self.eat_dec_digits() {
                        return Int {
                            base,
                            empty_int: true,
                        };
                    }
                }
                'x' | 'X' => {
                    base = Hex;
                    self.bump();
                    if !self.eat_hex_digits() {
                        return Int {
                            base,
                            empty_int: true,
                        };
                    }
                }
                // Not a base prefix; eat additional digits
                '0'..='9' | '_' => {
                    self.eat_dec_digits();
                }
                // Also not a base prefix; nothing more to do here.
                '.' | 'e' | 'E' => {}
                // Just a 0.
                _ => {
                    return Int {
                        base,
                        empty_int: false,
                    };
                }
            }
        } else {
            self.eat_dec_digits();
        }

        match self.first() {
            '.' if !is_ident_start(self.second()) => {
                // might have stuff after the ., and if it does, it needs to start
                // with a number
                self.bump();
                let mut empty_exp = false;
                if self.first().is_ascii_digit() {
                    self.eat_dec_digits();
                    match self.first() {
                        'e' | 'E' => {
                            self.bump();
                            empty_exp = !self.eat_float_exp();
                        }
                        _ => (),
                    }
                }
                Float { base, empty_exp }
            }
            'e' | 'E' => {
                self.bump();
                let empty_exp = !self.eat_float_exp();
                Float { base, empty_exp }
            }
            _ => Int {
                base,
                empty_int: false,
            },
        }
    }

    fn eat_dec_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '_' => {
                    self.bump();
                }
                '0'..='9' => {
                    has_digits = true;
                    self.bump();
                }
                _ => return has_digits,
            }
        }
    }

    fn eat_hex_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '_' => {
                    self.bump();
                }
                c if c.is_ascii_hexdigit() => {
                    has_digits = true;
                    self.bump();
                }
                _ => return has_digits,
            }
        }
    }

    /// Eats the float exponent. Returns true if at least one digit was met,
    /// and returns false otherwise.
    fn eat_float_exp(&mut self) -> bool {
        if self.first() == '+' || self.first() == '-' {
            self.bump();
        }

        self.eat_dec_digits()
    }

    fn raw_ident(&mut self) -> TokenKind {
        self.bump();

        let mut terminated = false;
        while let Some(c) = self.bump() {
            match c {
                '`' => {
                    terminated = true;
                    break;
                }
                '\u{000A}' | '\u{000D}' => break,
                _ => continue,
            }
        }

        RawIdent { terminated }
    }

    fn ident(&mut self) -> TokenKind {
        self.bump();
        self.eat_while(is_ident_continue);

        Ident
    }
}

fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        | '\u{0020}' // space
        | '\u{0009}' // tab
        | '\u{000C}' // form feed
    )
}

fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_xid_start()
}

fn is_ident_continue(c: char) -> bool {
    c.is_xid_continue()
}
