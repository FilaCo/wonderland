use std::path::PathBuf;

use ac_db::db::AcDbTrait;
use ac_ir::{
    source::{SourceFile, Span},
    syntax::{Token, TokenKind},
};

pub(crate) fn dummy_token(db: &dyn AcDbTrait) -> Token<'_> {
    Token::new(db, TokenKind::Quest, dummy_span(db))
}

pub(crate) fn dummy_span(db: &dyn AcDbTrait) -> Span<'_> {
    Span::new(db, 0, 0, dummy_source_file(db))
}

pub(crate) fn dummy_source_file(db: &dyn AcDbTrait) -> SourceFile {
    SourceFile::new(db, PathBuf::from(""), String::from(""))
}
