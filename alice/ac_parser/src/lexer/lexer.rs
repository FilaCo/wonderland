use TokenKind::*;
use ac_db::db::AliceDatabaseTrait;
use ac_diag::Diagnostic;
use ac_ir::{
    source::{SourceFile, Span},
    syntax::{Base, LiteralKind, Symbol, Token, TokenKind},
};

use crate::{
    lexer::cursor::{Cursor, TokenKind as CursorTokenKind},
    util::dummy_token,
};

pub struct Lexer<'db> {
    db: &'db dyn AliceDatabaseTrait,
    file: SourceFile,
    cursor: Cursor<'db>,
    token: Token<'db>,
    pos: usize,
}

impl<'db> Lexer<'db> {
    pub fn new(db: &'db dyn AliceDatabaseTrait, file: SourceFile) -> Self {
        let mut lexer = Self {
            db,
            file,
            cursor: Cursor::new(file.contents(db)),
            token: dummy_token(db, file),
            pos: 0,
        };

        lexer.bump();
        lexer.pos = 0;

        lexer
    }

    pub fn peek(&self) -> &Token<'db> {
        &self.token
    }

    pub fn bump(&mut self) -> Token<'db> {
        let next_tok = loop {
            let (next_tok, is_next_tok_preceded_by_ws) = self.next_token_from_cursor();

            if is_next_tok_preceded_by_ws {
                break next_tok;
            } else if let Some(glued) = self.token.glue(&next_tok, self.db) {
                self.token = glued;
            } else {
                break next_tok;
            }
        };
        std::mem::replace(&mut self.token, next_tok)
    }

    fn next_token_from_cursor(&mut self) -> (Token<'db>, bool) {
        let mut preceded_by_ws = false;
        let mut swallow_next_invalid = false;

        // Skip whitespace & comments tokens
        loop {
            let str_before = self.cursor.as_str();
            let cursor_tok = self.cursor.advance_token();
            let start = self.pos;
            self.pos += cursor_tok.len;

            let kind = match cursor_tok.kind {
                CursorTokenKind::BlockComment { terminated } => {
                    if !terminated {
                        todo!() // TODO: diag
                    }
                    preceded_by_ws = true;
                    continue;
                }
                CursorTokenKind::LineComment => {
                    preceded_by_ws = true;
                    continue;
                }
                CursorTokenKind::WS => {
                    preceded_by_ws = true;
                    continue;
                }
                CursorTokenKind::NL => todo!(),
                CursorTokenKind::And => And,
                CursorTokenKind::Comma => Comma,
                CursorTokenKind::Colon => Colon,
                CursorTokenKind::Dot => Dot,
                CursorTokenKind::Eq => Eq,
                CursorTokenKind::Excl => Excl,
                CursorTokenKind::GT => GT,
                CursorTokenKind::LT => LT,
                CursorTokenKind::Minus => Minus,
                CursorTokenKind::Or => Or,
                CursorTokenKind::Percent => Percent,
                CursorTokenKind::Plus => Plus,
                CursorTokenKind::Quest => Quest,
                CursorTokenKind::Semi => Semi,
                CursorTokenKind::Slash => Slash,
                CursorTokenKind::Star => Star,
                CursorTokenKind::Tilde => Tilde,

                CursorTokenKind::LBrace => LBrace,
                CursorTokenKind::RBrace => RBrace,
                CursorTokenKind::LParen => LParen,
                CursorTokenKind::RParen => RParen,

                CursorTokenKind::RawIdent { terminated } => todo!(),
                CursorTokenKind::Ident => todo!(),

                CursorTokenKind::Literal { kind } => todo!(),

                CursorTokenKind::Unknown => todo!(),
                CursorTokenKind::EOF => EOF,
            };
        }
        todo!()
    }

    fn make_span(&self, start: usize, end: usize) -> Span<'db> {
        Span::new(self.db, start, end, self.file)
    }
}
