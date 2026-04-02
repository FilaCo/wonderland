use ac_db::db::AcDbTrait;
use ac_ir::{source::SourceFile, syntax::Token};

use crate::{Lexer, dummy_token};

pub struct Parser<'db> {
    pub(super) db: &'db dyn AcDbTrait,
    pub(super) file: SourceFile,
    pub(super) lexer: Lexer<'db>,
    pub(super) prev: Token<'db>,
    pub(super) cur: Token<'db>,
}

impl<'db> Parser<'db> {
    pub fn new(db: &'db dyn AcDbTrait, file: SourceFile) -> Self {
        Self {
            db,
            lexer: Lexer::new(db, file.contents(db)),
            file,
            prev: dummy_token(db),
            cur: dummy_token(db),
        }
    }

    pub(super) fn bump(&mut self) {
        self.prev = std::mem::replace(&mut self.cur, self.lexer.advance_token());
    }
}
