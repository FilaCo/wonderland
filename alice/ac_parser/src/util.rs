use ac_db::db::AliceDatabaseTrait;
use ac_ir::{
    source::{SourceFile, Span},
    syntax::{Token, TokenKind},
};

pub fn dummy_token(db: &dyn AliceDatabaseTrait, file: SourceFile) -> Token<'_> {
    Token::new(TokenKind::Quest, Span::new(db, 0, 0, file))
}
