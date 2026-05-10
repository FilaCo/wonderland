use crate::source::Span;

use TokenKind::*;
use salsa::Database;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub struct Token<'db> {
    pub kind: TokenKind<'db>,
    pub span: Span<'db>,
}

impl<'db> Token<'db> {
    pub const fn new(kind: TokenKind<'db>, span: Span<'db>) -> Self {
        Self { kind, span }
    }

    pub const fn eof(span: Span<'db>) -> Self {
        Self::new(EOF, span)
    }

    pub fn is_eof(&self) -> bool {
        self.kind == EOF
    }

    pub fn glue(&self, joint: &Self, db: &'db dyn Database) -> Option<Self> {
        let kind = match (self.kind, joint.kind) {
            (And, And) => Conj,
            (Colon, Colon) => ColonColon,
            (Or, Or) => Disj,
            (Dot, Dot) => DotDot,
            (Eq, Eq) => EqEq,
            (GT, Eq) => GE,
            (LT, Eq) => LE,
            (Minus, Eq) => MinusEq,
            (Excl, Eq) => NE,
            (Percent, Eq) => PercentEq,
            (Or, GT) => Pipe,
            (Plus, Eq) => PlusEq,
            (Slash, Eq) => SlashEq,
            (Star, Eq) => StarEq,
            _ => return None,
        };

        Some(Token::new(kind, self.span.to(&joint.span, db)))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub enum TokenKind<'db> {
    /// `LF | ( CR [ LF ] )`
    NL,
    /// `&`
    And,
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `.`
    Dot,
    /// `=`
    Eq,
    /// `!`
    Excl,
    /// `>`
    GT,
    /// `<`
    LT,
    /// `-`
    Minus,
    /// `|`
    Or,
    /// `%`
    Percent,
    /// `+`
    Plus,
    /// `?`
    Quest,
    /// `;`
    Semi,
    /// `/`
    Slash,
    /// `*`
    Star,
    /// `~`
    Tilde,

    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `(`
    LParen,
    /// `)`
    RParen,

    /// `&&`
    Conj,
    /// `::`
    ColonColon,
    /// `||`
    Disj,
    /// `..`
    DotDot,
    /// `==`
    EqEq,
    /// `>=`
    GE,
    /// `<=`
    LE,
    /// `-=`
    MinusEq,
    /// `!=`
    NE,
    /// `%=`
    PercentEq,
    /// `|>`
    Pipe,
    /// `+=`
    PlusEq,
    /// `/=`
    SlashEq,
    /// `*=`
    StarEq,

    /// `const`
    Const,
    /// `derive`
    Derive,
    /// `despawn`
    Despawn,
    /// `erase`
    Erase,
    /// `filter`
    Filter,
    /// `from`
    From,
    /// `in`
    In,
    /// `into`
    Into,
    /// `insert`
    Insert,
    /// `match`
    Match,
    /// `mod`
    Mod,
    /// `mut`
    Mut,
    /// `prop`
    Prop,
    /// `query`
    Query,
    /// `spawn`
    Spawn,
    /// `use`
    Use,
    /// `with`
    With,
    /// `without`
    Without,

    /// An escaped identifier, e.g. `` `ident` ``.
    RawIdent { symbol: Symbol<'db> },
    /// An identifier e.g. `ident`.
    Ident { symbol: Symbol<'db> },

    /// A literal constant value, e.g. `123` or `"hello"`.
    Literal {
        kind: LiteralKind,
        symbol: Symbol<'db>,
    },

    /// End of input.
    EOF,
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
