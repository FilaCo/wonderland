use ac_ir::syntax::LiteralKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    /// A block comment, e.g. `/* block comment */`.
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    ///
    /// `"/*" { BlockComment | /* an arbitrary Unicode code point */ } "*/" .`
    BlockComment { terminated: bool },
    /// A line comment, e.g. `// comment`.
    ///
    /// `"//" { /* an arbitrary Unicode code point except LF and CR */ } .`
    LineComment,
    /// `/* one of the following Unicode code points: SPACE U+0020, TAB U+0009, Form Feed U+000C */ .`
    WS,

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

    /// An escaped identifier, e.g. `` `ident` ``.
    RawIdent { terminated: bool },
    /// An identifier or keyword, e.g. `ident` or `prop`.
    Ident,

    /// A literal constant value, e.g. `123` or `"hello"`.
    Literal { kind: LiteralKind },

    /// Unknown token, not expected by the lexer, e.g. "№".
    Unknown,

    /// End of input.
    EOF,
}
