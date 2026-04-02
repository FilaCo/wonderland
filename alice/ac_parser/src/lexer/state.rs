use ac_db::db::AcDbTrait;
use ac_ir::syntax::Token;

use crate::{dummy_token, lexer::Cursor};

pub struct Lexer<'db> {
    db: &'db dyn AcDbTrait,
    cursor: Cursor<'db>,
    token: Token<'db>,
    pos: usize,
}

impl<'db> Lexer<'db> {
    pub fn new(db: &'db dyn AcDbTrait, input: &'db str) -> Self {
        Self {
            db,
            cursor: Cursor::new(input),
            token: dummy_token(db),
            pos: 0,
        }
    }
}
