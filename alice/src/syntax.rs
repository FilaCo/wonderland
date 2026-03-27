use chumsky::{
    input::{BorrowInput, ValueInput},
    pratt::{infix, left, prefix},
    prelude::*,
    primitive::select_ref,
};
use logos::Logos;

use Base::*;
use LitKind::*;

use crate::source::{SourceFileId, Span, Spanned};

pub fn alice_script<'src, I>(
    file_id: SourceFileId,
) -> impl Parser<'src, I, AliceScript<'src>, extra::Err<Rich<'src, Token<'src>, Span>>>
where
    I: ValueInput<'src, Span = Span, Token = Token<'src>>,
{
    let expr = recursive(|expr| {
        let lit = select! {
            Token::Lit(data) => ExprKind::Lit { data }
        };
        // let grouped = expr.delimited_by(just(LParen), just(RParen)).map(|e| e);
        let atom = lit;

        atom.pratt((prefix(2, just(Token::Minus), |op, rhs, e| {
            ExprKind::Unary {
                op,
                rhs: Box::new((rhs, e.span())),
            }
        }),))
    });

    let stmt = expr.map_with(|kind, e| (StatementKind::Expr(kind), e.span()));

    stmt.repeated()
        .collect::<Vec<_>>()
        .map(|stmts| AliceScript { stmts })
}

#[derive(Clone, Debug)]
pub struct AliceScript<'src> {
    pub stmts: Vec<Spanned<StatementKind<'src>>>,
}

#[derive(Clone, Debug)]
pub enum StatementKind<'src> {
    Expr(ExprKind<'src>),
}

#[derive(Clone, Debug)]
pub enum ExprKind<'src> {
    Binary {
        lhs: Box<Spanned<ExprKind<'src>>>,
        op: Token<'src>,
        rhs: Box<Spanned<ExprKind<'src>>>,
    },
    Unary {
        op: Token<'src>,
        rhs: Box<Spanned<ExprKind<'src>>>,
    },
    Grouped {
        expr: Box<Spanned<ExprKind<'src>>>,
    },
    Lit {
        data: LitData<'src>,
    },
}

#[derive(Logos, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token<'src> {
    /// `+`
    #[token("+")]
    Plus,
    /// `-`
    #[token("-")]
    Minus,
    /// `*`
    #[token("*")]
    Star,
    /// `/`
    #[token("/")]
    Slash,

    /// `(`
    #[token("(")]
    LParen,
    /// `)`
    #[token(")")]
    RParen,

    #[regex(r"0(b|B)[01][01_]*", |lex| LitData::bin_lit(lex.slice()))]
    #[regex(r"0(o|O)[0-7][0-7_]*", |lex| LitData::oct_lit(lex.slice()))]
    #[regex(r"[0-9][0-9_]*", |lex| LitData::dec_int_lit(lex.slice()))]
    #[regex(r"0(x|X)[0-9a-fA-F][0-9a-fA-F_]*", |lex| LitData::hex_int_lit(lex.slice()))]
    Lit(LitData<'src>),

    #[regex(r"[ \r\n\t]+", logos::skip)]
    Whitespace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LitData<'src> {
    pub kind: LitKind,
    pub symbol: &'src str,
}

impl<'src> LitData<'src> {
    pub const fn new(kind: LitKind, symbol: &'src str) -> Self {
        Self { kind, symbol }
    }
    pub const fn bin_lit(symbol: &'src str) -> Self {
        Self::new(Int { base: Bin }, symbol)
    }
    pub const fn oct_lit(symbol: &'src str) -> Self {
        Self::new(Int { base: Oct }, symbol)
    }
    pub const fn dec_int_lit(symbol: &'src str) -> Self {
        Self::new(Int { base: Dec }, symbol)
    }
    pub const fn hex_int_lit(symbol: &'src str) -> Self {
        Self::new(Int { base: Hex }, symbol)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LitKind {
    Int { base: Base },
    Float { base: Base },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Base {
    Bin = 2,
    Oct = 8,
    Dec = 10,
    Hex = 16,
}
