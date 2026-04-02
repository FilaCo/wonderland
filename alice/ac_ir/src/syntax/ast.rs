use crate::{
    source::Span,
    syntax::{Symbol, Token},
};

#[salsa::tracked(debug)]
pub struct AliceFile<'db> {
    #[tracked]
    #[returns(ref)]
    pub top_level_stmts: Vec<TopLevelStatement<'db>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub struct TopLevelStatement<'db> {
    pub span: Span<'db>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub struct Expr<'db> {
    pub kind: ExprKind<'db>,
    pub span: Span<'db>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub enum ExprKind<'db> {
    Binary {
        lhs: Box<Expr<'db>>,
        op: Token<'db>,
        rhs: Box<Expr<'db>>,
    },
    Unary {
        op: Token<'db>,
        rhs: Box<Expr<'db>>,
    },
    Grouped {
        expr: Box<Expr<'db>>,
    },
    LiteralConst {
        value: Token<'db>,
    },
}
