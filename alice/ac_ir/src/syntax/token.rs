use crate::source::Span;

#[salsa::tracked(debug)]
pub struct Token<'db> {
    pub kind: TokenKind<'db>,
    pub span: Span<'db>,
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
    Int { base: Base },
    Float { base: Base },
    Rune { terminated: bool },
    Str { terminated: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub enum Base {
    Bin = 2,
    Oct = 8,
    Dec = 10,
    Hex = 16,
}

#[salsa::interned(debug)]
pub struct Symbol<'db> {
    #[returns(ref)]
    pub value: String,
}
