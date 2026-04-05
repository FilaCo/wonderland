use TokenKind::*;
use ac_db::db::AliceDatabaseTrait;
use ac_ir::{
    source::{SourceFile, Span},
    syntax::{Base, LiteralKind, Symbol, Token, TokenKind},
};

use crate::{
    lexer::{self, Cursor},
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

        let _ = lexer.bump();

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
                lexer::TokenKind::BlockComment { terminated } => {
                    if !terminated {
                        todo!() // TODO: diag
                    }
                    preceded_by_ws = true;
                    continue;
                }
                lexer::TokenKind::LineComment => {
                    preceded_by_ws = true;
                    continue;
                }
                lexer::TokenKind::NewLine => todo!(),
                lexer::TokenKind::Whitespace => {
                    preceded_by_ws = true;
                    continue;
                }
                lexer::TokenKind::Eq => Eq,
                lexer::TokenKind::Lt => Lt,
                lexer::TokenKind::Gt => Gt,
                lexer::TokenKind::Excl => Excl,
                lexer::TokenKind::Plus => Plus,
                lexer::TokenKind::Minus => Minus,
                lexer::TokenKind::Star => Star,
                lexer::TokenKind::Slash => Slash,
                lexer::TokenKind::Dot => Dot,
                lexer::TokenKind::Comma => Comma,
                lexer::TokenKind::Semi => Semi,
                lexer::TokenKind::Colon => Colon,
                lexer::TokenKind::Quest => Quest,
                lexer::TokenKind::Pipe => Pipe,
                lexer::TokenKind::LBrace => LBrace,
                lexer::TokenKind::RBrace => RBrace,
                lexer::TokenKind::LParen => LParen,
                lexer::TokenKind::RParen => RParen,
                lexer::TokenKind::Literal { kind } => todo!(),
                lexer::TokenKind::Ident => todo!(),
                lexer::TokenKind::Unknown => todo!(),
                lexer::TokenKind::EndOfInput => EndOfInput,
            };
        }
        todo!()
    }

    fn make_span(&self, start: usize, end: usize) -> Span<'db> {
        Span::new(self.db, start, end, self.file)
    }
}
