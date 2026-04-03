use crate::source::Span;

use TokenKind::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub struct Token<'db> {
    pub kind: TokenKind<'db>,
    pub span: Span<'db>,
}

impl<'db> Token<'db> {
    pub const fn new(kind: TokenKind<'db>, span: Span<'db>) -> Self {
        Self { kind, span }
    }

    pub const fn eoi(span: Span<'db>) -> Self {
        Self::new(EndOfInput, span)
    }

    pub fn is_eoi(&self) -> bool {
        self.kind == EndOfInput
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub enum TokenKind<'db> {
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
    /// `Whitespace = ( " " | "\t" | "\f" ) { ( " " | "\t" | "\f" ) } .`
    Whitespace,
    /// `NewLine = LineFeed | ( CarriageReturn [ LineFeed ] )`
    NewLine,

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

    /// `==`
    EqEq,
    /// `!=`
    Ne,
    /// `<=`
    Le,
    /// `>=`
    Ge,
    /// `+=`
    PlusEq,
    /// `-=`
    MinusEq,
    /// `*=`
    StarEq,
    /// `/=`
    SlashEq,
    /// `::`
    ColonColon,

    /// A literal constant value, e.g. `123` or `"hello"`.
    Literal {
        kind: LiteralKind,
        symbol: Symbol<'db>,
    },

    /// An identifier or keyword, e.g. `ident` or `prop`.
    Ident { symbol: Symbol<'db> },

    /// Unknown token, not expected by the lexer, e.g. "№".
    Unknown,

    /// End of input.
    EndOfInput,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub enum LiteralKind {
    Int { base: Base, empty_int: bool },
    Float { base: Base, empty_exp: bool },
}

/// Base of numeric literal encoding according to its prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, salsa::Update)]
pub enum Base {
    /// Literal starts with "0b" or "0B".
    Bin = 2,
    /// Literal starts with "0o" or "0O".
    Oct = 8,
    /// Literal doesn't contain a prefix.
    Dec = 10,
    /// Literal starts with "0x" or "0X".
    Hex = 16,
}

#[salsa::interned(debug)]
pub struct Symbol<'db> {
    #[returns(ref)]
    pub value: String,
}
