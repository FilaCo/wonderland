use ac_db::db::AliceDatabaseTrait;
use ac_ir::{
    source::{SourceFile, Span},
    syntax::{Base, LiteralKind, Symbol, Token, TokenKind},
};
use unicode_xid::UnicodeXID;

use crate::{lexer::Cursor, util::dummy_token};
use LiteralKind::*;
use TokenKind::*;

pub struct Lexer<'db> {
    db: &'db dyn AliceDatabaseTrait,
    file: SourceFile,
    cursor: Cursor<'db>,
    token: Token<'db>,
    pos: usize,
}

impl<'db> Lexer<'db> {
    pub fn new(db: &'db dyn AliceDatabaseTrait, file: SourceFile) -> Self {
        let mut lexer = Self {
            db,
            file,
            cursor: Cursor::new(file.contents(db)),
            token: dummy_token(db, file),
            pos: 0,
        };

        let _ = lexer.advance_token();

        lexer
    }

    pub fn peek(&self) -> &Token<'db> {
        &self.token
    }

    pub fn advance_token(&mut self) -> Token<'db> {
        if self.is_at_eoi() {
            return self.token;
        }

        let Some(first_char) = self.cursor.bump() else {
            let span = self.make_span();
            return std::mem::replace(&mut self.token, Token::eoi(span));
        };

        let kind = match first_char {
            '/' => match self.cursor.first() {
                '/' => self.line_comment(),
                '*' => self.block_comment(),
                _ => Slash,
            },

            c if is_whitespace(c) => self.whitespace(),

            '\r' => match self.cursor.first() {
                '\n' => {
                    self.cursor.bump();
                    NewLine
                }
                _ => self.whitespace(),
            },
            '\n' => NewLine,

            '0'..='9' => {
                let kind = self.number(first_char);
                let symbol = self.make_symbol();
                Literal { kind, symbol }
            }

            c if is_ident_start(c) => self.ident(),

            '=' => Eq,
            '<' => Lt,
            '>' => Gt,
            '!' => Excl,
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '.' => Dot,
            ',' => Comma,
            ';' => Semi,
            ':' => Colon,
            '?' => Quest,
            '|' => Pipe,

            '{' => LBrace,
            '}' => RBrace,
            '(' => LParen,
            ')' => RParen,

            _ => Unknown,
        };

        todo!()
    }

    pub fn is_at_eoi(&self) -> bool {
        self.token.is_eoi()
    }

    fn block_comment(&mut self) -> TokenKind<'db> {
        self.cursor.bump();

        let mut depth = 1usize;
        while let Some(c) = self.cursor.bump() {
            match c {
                '/' if self.cursor.first() == '*' => {
                    self.cursor.bump();
                    depth += 1;
                }
                '*' if self.cursor.first() == '/' => {
                    self.cursor.bump();
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

    fn line_comment(&mut self) -> TokenKind<'db> {
        self.cursor.eat_until(b'\n');

        LineComment
    }

    fn whitespace(&mut self) -> TokenKind<'db> {
        self.cursor.eat_while(is_whitespace);

        Whitespace
    }

    fn number(&mut self, first_digit: char) -> LiteralKind {
        use Base::*;
        let mut base = Dec;
        if first_digit == '0' {
            match self.cursor.first() {
                'b' | 'B' => {
                    base = Bin;
                    self.cursor.bump();
                    if !self.eat_dec_digits() {
                        return Int {
                            base,
                            empty_int: true,
                        };
                    }
                }
                'o' | 'O' => {
                    base = Oct;
                    self.cursor.bump();
                    if !self.eat_dec_digits() {
                        return Int {
                            base,
                            empty_int: true,
                        };
                    }
                }
                'x' | 'X' => {
                    base = Hex;
                    self.cursor.bump();
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

        match self.cursor.first() {
            '.' if !is_ident_start(self.cursor.second()) => {
                // might have stuff after the ., and if it does, it needs to start
                // with a number
                self.cursor.bump();
                let mut empty_exp = false;
                if self.cursor.first().is_ascii_digit() {
                    self.eat_dec_digits();
                    match self.cursor.first() {
                        'e' | 'E' => {
                            self.cursor.bump();
                            empty_exp = !self.eat_float_exp();
                        }
                        _ => (),
                    }
                }
                Float { base, empty_exp }
            }
            'e' | 'E' => {
                self.cursor.bump();
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
            match self.cursor.first() {
                '_' => {
                    self.cursor.bump();
                }
                '0'..='9' => {
                    has_digits = true;
                    self.cursor.bump();
                }
                _ => return has_digits,
            }
        }
    }

    fn eat_hex_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.cursor.first() {
                '_' => {
                    self.cursor.bump();
                }
                c if c.is_ascii_hexdigit() => {
                    has_digits = true;
                    self.cursor.bump();
                }
                _ => return has_digits,
            }
        }
    }

    /// Eats the float exponent. Returns true if at least one digit was met,
    /// and returns false otherwise.
    fn eat_float_exp(&mut self) -> bool {
        if self.cursor.first() == '+' || self.cursor.first() == '-' {
            self.cursor.bump();
        }

        self.eat_dec_digits()
    }

    fn ident(&mut self) -> TokenKind<'db> {
        self.cursor.bump();
        self.cursor.eat_while(is_ident_continue);

        Ident {
            symbol: self.make_symbol(),
        }
    }

    fn glue(&mut self, kind: TokenKind<'db>) -> TokenKind<'db> {
        match kind {
            Eq if self.cursor.first() == '=' => {
                self.cursor.bump();
                EqEq
            }
            Lt if self.cursor.first() == '=' => {
                self.cursor.bump();
                Le
            }
            Gt if self.cursor.first() == '=' => {
                self.cursor.bump();
                Ge
            }
            Excl if self.cursor.first() == '=' => {
                self.cursor.bump();
                Ne
            }
            Plus if self.cursor.first() == '=' => {
                self.cursor.bump();
                PlusEq
            }
            Minus if self.cursor.first() == '=' => {
                self.cursor.bump();
                MinusEq
            }
            Star if self.cursor.first() == '=' => {
                self.cursor.bump();
                StarEq
            }
            Slash if self.cursor.first() == '=' => {
                self.cursor.bump();
                SlashEq
            }
            Colon if self.cursor.first() == ':' => {
                self.cursor.bump();
                ColonColon
            }
            _ => kind,
        }
    }

    fn make_span(&self) -> Span<'db> {
        todo!()
    }

    fn make_symbol(&self) -> Symbol<'db> {
        let src = self.file.contents(self.db);
        let start = self.pos;
        let end = self.pos + self.cursor.bumped_len();
        let value = String::from(&src[start..end]);

        Symbol::new(self.db, value)
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
