use ac_ir::syntax::Expr;

use crate::Parser;

impl<'db> Parser<'db> {
    pub(super) fn expr(&mut self) -> Expr<'db> {
        self.expr_bp(0)
    }

    fn expr_bp(&mut self, min_bp: u8) -> Expr<'db> {
        todo!()
    }
}
