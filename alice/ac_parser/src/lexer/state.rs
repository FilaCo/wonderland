use ac_db::db::AcDbTrait;
use ac_ir::syntax::Token;

use crate::{dummy_token, lexer::Cursor};

pub(crate) struct Lexer<'db> {
    pub(super) db: &'db dyn AcDbTrait,
    pub(super) cursor: Cursor<'db>,
    pub(super) token: Token<'db>,
    pub(super) pos: usize,
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
