use crate::{source::Spanned, syntax::token::Token};

#[derive(Clone, Debug)]
pub struct AliceScript<'src> {
    pub top_level_objects: Vec<Spanned<TopLevelObject<'src>>>,
}

#[derive(Clone, Debug)]
pub enum TopLevelObject<'src> {
    TopLevelStmt(TopLevelStmtKind<'src>),
    TopLevelDecl,
}

#[derive(Clone, Debug)]
pub enum TopLevelStmtKind<'src> {
    UsingNamespace { namespace_ident: &'src str },
}

#[derive(Clone, Debug)]
pub enum ExprKind<'src> {
    Binary {
        lhs: Box<Spanned<Self>>,
        op: Spanned<Token<'src>>,
        rhs: Box<Spanned<Self>>,
    },
    Unary {
        op: Spanned<Token<'src>>,
        rhs: Box<Spanned<Self>>,
    },
    Grouped {
        expr: Box<Spanned<Self>>,
    },
    Literal {
        value: Spanned<Token<'src>>,
    },
}
