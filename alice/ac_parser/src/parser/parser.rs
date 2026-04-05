use ac_db::db::AliceDatabaseTrait;
use ac_ir::{
    source::SourceFile,
    syntax::{Expr, Token},
};

use crate::{Lexer, util::dummy_token};

pub struct Parser<'db> {
    db: &'db dyn AliceDatabaseTrait,
    file: SourceFile,
    lexer: Lexer<'db>,
    prev: Token<'db>,
    cur: Token<'db>,
}

impl<'db> Parser<'db> {
    pub fn new(db: &'db dyn AliceDatabaseTrait, file: SourceFile) -> Self {
        let mut parser = Self {
            db,
            lexer: Lexer::new(db, file),
            file,
            prev: dummy_token(db, file),
            cur: dummy_token(db, file),
        };

        parser.bump();

        parser
    }

    fn expr(&mut self) -> Expr<'db> {
        self.expr_bp(0)
    }

    fn expr_bp(&mut self, min_bp: u8) -> Expr<'db> {
        let mut lhs = match self.cur.kind {
            _ => todo!(),
        };
        todo!()
    }

    fn bump(&mut self) {
        self.prev = std::mem::replace(&mut self.cur, self.lexer.bump());
    }
}
