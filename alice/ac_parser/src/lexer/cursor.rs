use std::str::Chars;

use LiteralKind::*;
use TokenKind::*;
use ac_ir::syntax::{Base, LiteralKind};
use memchr::memchr;
use unicode_xid::UnicodeXID;

/// Peekable iterator over a char sequence.
///
/// Next characters can be peeked via `first` method,
/// and position can be shifted forward via `bump` method.
#[derive(Debug)]
pub(super) struct Cursor<'src> {
    len_remaining: usize,
    /// Iterator over chars. Slightly faster than a &str.
    chars: Chars<'src>,
}

impl<'src> Cursor<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            chars: input.chars(),
            len_remaining: input.len(),
        }
    }

    pub fn advance_token(&mut self) -> Token {
        let Some(first_char) = self.bump() else {
            return Token {
                kind: EndOfInput,
                len: 0,
            };
        };

        let kind = match first_char {
            '/' => match self.first() {
                '/' => self.line_comment(),
                '*' => self.block_comment(),
                _ => Slash,
            },

            '\r' => match self.first() {
                '\n' => {
                    self.bump();

                    NewLine
                }
                _ => self.whitespace(),
            },
            '\n' => NewLine,

            c if is_whitespace(c) => self.whitespace(),

            '0'..='9' => Literal {
                kind: self.number(first_char),
            },

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

        let len = self.bumped_len();
        self.reset_len_remaining();

        Token::new(kind, len)
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

    fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);

        Whitespace
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

    fn ident(&mut self) -> TokenKind {
        self.bump();
        self.eat_while(is_ident_continue);

        Ident
    }

    pub fn as_str(&self) -> &'src str {
        self.chars.as_str()
    }

    /// Peeks the next symbol from the input stream without consuming it.
    /// If requested position doesn't exist, `EOF_CHAR` is returned.
    /// However, getting `EOI_CHAR` doesn't always mean actual end of file,
    /// it should be checked with `is_eoI` method.
    pub fn first(&self) -> char {
        // `.next()` optimizes better than `.nth(0)`
        self.chars.clone().next().unwrap_or(EOI_CHAR)
    }

    /// Peeks the second symbol from the input stream without consuming it.
    pub fn second(&self) -> char {
        // `.next()` optimizes better than `.nth(1)`
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOI_CHAR)
    }

    /// Moves to the next character.
    pub fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Eats symbols while predicate returns true or until the end of file is reached.
    pub fn eat_while(&mut self, predicate: impl Fn(char) -> bool) {
        while predicate(self.first()) && !self.is_at_eoi() {
            self.bump();
        }
    }

    pub fn eat_until(&mut self, byte: u8) {
        self.chars = match memchr(byte, self.as_str().as_bytes()) {
            Some(index) => self.as_str()[index..].chars(),
            None => "".chars(),
        }
    }

    /// Returns amount of already bumped symbols.
    pub fn bumped_len(&self) -> usize {
        self.len_remaining - self.chars.as_str().len()
    }

    /// Resets the number of bytes consumed to 0.
    pub fn reset_len_remaining(&mut self) {
        self.len_remaining = self.chars.as_str().len();
    }

    /// Checks if there is nothing more to consume.
    pub fn is_at_eoi(&self) -> bool {
        self.chars.as_str().is_empty()
    }
}

const EOI_CHAR: char = '\0';
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub const fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }
    pub const fn eoi() -> Self {
        Self::new(EndOfInput, 0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum TokenKind {
    /// A block comment, e.g. `/* block comment */`.
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    ///
    /// `BlockComment = "/*" { BlockComment | <any character> } "*/" .`
    BlockComment { terminated: bool },
    /// A line comment, e.g. `// comment`.
    ///
    /// `LineComment = "//" { <any character except CarriageReturn and LineFeed> } .`
    LineComment,
    /// `NewLine = LineFeed | ( CarriageReturn [ LineFeed ] )`
    NewLine,
    /// `Whitespace = ( " " | "\t" | "\f" ) { ( " " | "\t" | "\f" ) } .`
    Whitespace,

    /// `=`
    Eq,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `!`
    Excl,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `.`
    Dot,
    /// `,`
    Comma,
    /// `;`
    Semi,
    /// `:`
    Colon,
    /// `?`
    Quest,
    /// `|`
    Pipe,

    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `(`
    LParen,
    /// `)`
    RParen,

    /// A literal constant value, e.g. `123` or `"hello"`.
    Literal { kind: LiteralKind },

    /// An identifier or keyword, e.g. `ident` or `prop`.
    Ident,

    /// Unknown token, not expected by the lexer, e.g. "№".
    Unknown,

    /// End of input.
    EndOfInput,
}
