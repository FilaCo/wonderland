use super::*;
use expect_test::{Expect, expect};

fn check(input: &str, expected: Expect) {
    let tokens: Vec<Token> = Cursor::tokenize(input).collect();
    expected.assert_debug_eq(&tokens);
}

#[test]
fn tokenize_empty_input() {
    check(
        "",
        expect![[r#"
        []
    "#]],
    );
}

#[test]
fn tokenize_only_whitespace() {
    check(
        "   \t\u{000C} ",
        expect![[r#"
        [
            Token {
                kind: WS,
                len: 6,
            },
        ]
    "#]],
    );
}

#[test]
fn tokenize_only_newlines_and_crlf() {
    check(
        "\n\r\n\r\n\r",
        expect![[r#"
        [
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: NL,
                len: 2,
            },
            Token {
                kind: NL,
                len: 2,
            },
            Token {
                kind: WS,
                len: 1,
            },
        ]
    "#]],
    );
}

#[test]
fn tokenize_unknown_token() {
    check(
        "№ ?",
        expect![[r#"
        [
            Token {
                kind: Unknown,
                len: 3,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Quest,
                len: 1,
            },
        ]
    "#]],
    );
}

#[test]
fn tokenize_integers_all_bases() {
    check(
        "0 1_000 0b101 0o7 0x1A_b 0B1 0O1 0X1 0b 0o 0x",
        expect![[r#"
            [
                Token {
                    kind: Literal {
                        kind: Int {
                            base: Dec,
                            empty_int: false,
                        },
                    },
                    len: 1,
                },
                Token {
                    kind: WS,
                    len: 1,
                },
                Token {
                    kind: Literal {
                        kind: Int {
                            base: Dec,
                            empty_int: false,
                        },
                    },
                    len: 5,
                },
                Token {
                    kind: WS,
                    len: 1,
                },
                Token {
                    kind: Literal {
                        kind: Int {
                            base: Bin,
                            empty_int: false,
                        },
                    },
                    len: 5,
                },
                Token {
                    kind: WS,
                    len: 1,
                },
                Token {
                    kind: Literal {
                        kind: Int {
                            base: Oct,
                            empty_int: false,
                        },
                    },
                    len: 3,
                },
                Token {
                    kind: WS,
                    len: 1,
                },
                Token {
                    kind: Literal {
                        kind: Int {
                            base: Hex,
                            empty_int: false,
                        },
                    },
                    len: 6,
                },
                Token {
                    kind: WS,
                    len: 1,
                },
                Token {
                    kind: Literal {
                        kind: Int {
                            base: Bin,
                            empty_int: false,
                        },
                    },
                    len: 3,
                },
                Token {
                    kind: WS,
                    len: 1,
                },
                Token {
                    kind: Literal {
                        kind: Int {
                            base: Oct,
                            empty_int: false,
                        },
                    },
                    len: 3,
                },
                Token {
                    kind: WS,
                    len: 1,
                },
                Token {
                    kind: Literal {
                        kind: Int {
                            base: Hex,
                            empty_int: false,
                        },
                    },
                    len: 3,
                },
                Token {
                    kind: WS,
                    len: 1,
                },
                Token {
                    kind: Literal {
                        kind: Int {
                            base: Bin,
                            empty_int: true,
                        },
                    },
                    len: 2,
                },
                Token {
                    kind: WS,
                    len: 1,
                },
                Token {
                    kind: Literal {
                        kind: Int {
                            base: Oct,
                            empty_int: true,
                        },
                    },
                    len: 2,
                },
                Token {
                    kind: WS,
                    len: 1,
                },
                Token {
                    kind: Literal {
                        kind: Int {
                            base: Hex,
                            empty_int: true,
                        },
                    },
                    len: 2,
                },
            ]
        "#]],
    );
}

#[test]
fn tokenize_floats_and_exponents() {
    check(
        "1.5 1e10 1.5e+10 1e 1.",
        expect![[r#"
        [
            Token {
                kind: Literal {
                    kind: Float {
                        base: Dec,
                        empty_exp: false,
                    },
                },
                len: 3,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Literal {
                    kind: Float {
                        base: Dec,
                        empty_exp: false,
                    },
                },
                len: 4,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Literal {
                    kind: Float {
                        base: Dec,
                        empty_exp: false,
                    },
                },
                len: 7,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Literal {
                    kind: Float {
                        base: Dec,
                        empty_exp: true,
                    },
                },
                len: 2,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Literal {
                    kind: Float {
                        base: Dec,
                        empty_exp: false,
                    },
                },
                len: 2,
            },
        ]
    "#]],
    );
}

#[test]
fn tokenize_int_dot_ident_does_not_become_float() {
    check(
        "1.foo",
        expect![[r#"
        [
            Token {
                kind: Literal {
                    kind: Int {
                        base: Dec,
                        empty_int: false,
                    },
                },
                len: 1,
            },
            Token {
                kind: Dot,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 3,
            },
        ]
    "#]],
    );
}

#[test]
fn tokenize_line_and_block_comments() {
    check(
        "// line comment\n/* block */ /",
        expect![[r#"
        [
            Token {
                kind: LineComment,
                len: 15,
            },
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: BlockComment {
                    terminated: true,
                },
                len: 11,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Slash,
                len: 1,
            },
        ]
    "#]],
    );
}

#[test]
fn tokenize_nested_and_unterminated_block_comments() {
    check(
        "/* /* nested */ */\n/* */ */\n/* unterminated",
        expect![[r#"
            [
                Token {
                    kind: BlockComment {
                        terminated: true,
                    },
                    len: 18,
                },
                Token {
                    kind: NL,
                    len: 1,
                },
                Token {
                    kind: BlockComment {
                        terminated: true,
                    },
                    len: 5,
                },
                Token {
                    kind: WS,
                    len: 1,
                },
                Token {
                    kind: Star,
                    len: 1,
                },
                Token {
                    kind: Slash,
                    len: 1,
                },
                Token {
                    kind: NL,
                    len: 1,
                },
                Token {
                    kind: BlockComment {
                        terminated: false,
                    },
                    len: 15,
                },
            ]
        "#]],
    );
}

#[test]
fn tokenize_doc_comment_with_multibyte() {
    check(
        "// 界界\n/* 界 */",
        expect![[r#"
        [
            Token {
                kind: LineComment,
                len: 9,
            },
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: BlockComment {
                    terminated: true,
                },
                len: 9,
            },
        ]
    "#]],
    );
}

#[test]
fn tokenize_idents_ascii_and_underscore() {
    check(
        "foo _bar a1b2 _ Foo",
        expect![[r#"
        [
            Token {
                kind: Ident,
                len: 3,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 4,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 4,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 5,
            },
        ]
    "#]],
    );
}

#[test]
fn tokenize_idents_unicode_xid() {
    check(
        "αβγ привет 界面",
        expect![[r#"
        [
            Token {
                kind: Ident,
                len: 6,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 12,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 6,
            },
        ]
    "#]],
    );
}

#[test]
fn tokenize_raw_idents_terminated_and_unterminated() {
    check(
        "`foo` `prop` `with space` `multi\nline` `unterm",
        expect![[r#"
            [
                Token {
                    kind: RawIdent {
                        terminated: true,
                    },
                    len: 5,
                },
                Token {
                    kind: WS,
                    len: 1,
                },
                Token {
                    kind: RawIdent {
                        terminated: true,
                    },
                    len: 6,
                },
                Token {
                    kind: WS,
                    len: 1,
                },
                Token {
                    kind: RawIdent {
                        terminated: true,
                    },
                    len: 12,
                },
                Token {
                    kind: WS,
                    len: 1,
                },
                Token {
                    kind: RawIdent {
                        terminated: false,
                    },
                    len: 7,
                },
                Token {
                    kind: Ident,
                    len: 4,
                },
                Token {
                    kind: RawIdent {
                        terminated: true,
                    },
                    len: 3,
                },
                Token {
                    kind: Ident,
                    len: 6,
                },
            ]
        "#]],
    );
}

#[test]
fn tokenize_all_single_char_punctuation() {
    check(
        "& , : . = ! > < - | % + ? ; / * ~ { } ( )",
        expect![[r#"
        [
            Token {
                kind: And,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Comma,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Colon,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Dot,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Eq,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Excl,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: GT,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: LT,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Minus,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Or,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Percent,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Plus,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Quest,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Semi,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Slash,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Star,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Tilde,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: LBrace,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: RBrace,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: LParen,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: RParen,
                len: 1,
            },
        ]
    "#]],
    );
}

#[test]
fn tokenize_program_use_stmt() {
    check(
        "use core::math::*",
        expect![[r#"
        [
            Token {
                kind: Ident,
                len: 3,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 4,
            },
            Token {
                kind: Colon,
                len: 1,
            },
            Token {
                kind: Colon,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 4,
            },
            Token {
                kind: Colon,
                len: 1,
            },
            Token {
                kind: Colon,
                len: 1,
            },
            Token {
                kind: Star,
                len: 1,
            },
        ]
    "#]],
    );
}

#[test]
fn tokenize_program_const_decl() {
    let src = "const PI: f64 = 3.14159\nconst MAX_HP = 100";
    check(
        src,
        expect![[r#"
        [
            Token {
                kind: Ident,
                len: 5,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 2,
            },
            Token {
                kind: Colon,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 3,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Eq,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Literal {
                    kind: Float {
                        base: Dec,
                        empty_exp: false,
                    },
                },
                len: 7,
            },
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 5,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 6,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Eq,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Literal {
                    kind: Int {
                        base: Dec,
                        empty_int: false,
                    },
                },
                len: 3,
            },
        ]
    "#]],
    );
}

#[test]
fn tokenize_program_record_prop() {
    let src = "prop Position {\n    x: f32;\n    y: f32;\n}";
    check(
        src,
        expect![[r#"
        [
            Token {
                kind: Ident,
                len: 4,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 8,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: LBrace,
                len: 1,
            },
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: WS,
                len: 4,
            },
            Token {
                kind: Ident,
                len: 2,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 3,
            },
            Token {
                kind: Semi,
                len: 1,
            },
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: WS,
                len: 4,
            },
            Token {
                kind: Ident,
                len: 2,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 3,
            },
            Token {
                kind: Semi,
                len: 1,
            },
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: RBrace,
                len: 1,
            },
        ]
    "#]],
    );
}

#[test]
fn tokenize_program_enum_prop() {
    let src = "prop State =\n    | Idle\n    | Running(f32)\n    | Dead";
    check(
        src,
        expect![[r#"
        [
            Token {
                kind: Ident,
                len: 4,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 5,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Eq,
                len: 1,
            },
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: WS,
                len: 4,
            },
            Token {
                kind: Or,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 4,
            },
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: WS,
                len: 4,
            },
            Token {
                kind: Or,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 7,
            },
            Token {
                kind: LParen,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 3,
            },
            Token {
                kind: RParen,
                len: 1,
            },
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: WS,
                len: 4,
            },
            Token {
                kind: Or,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 4,
            },
        ]
    "#]],
    );
}

#[test]
fn tokenize_program_system_with_pipeline() {
    let src = "in Update {\n    query (mut pos: Position, vel: Velocity)\n    | filter pos.x > 0\n    | derive pos.x = pos.x + vel.x\n}";
    check(
        src,
        expect![[r#"
        [
            Token {
                kind: Ident,
                len: 2,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 6,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: LBrace,
                len: 1,
            },
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: WS,
                len: 4,
            },
            Token {
                kind: Ident,
                len: 5,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: LParen,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 3,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 3,
            },
            Token {
                kind: Colon,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 8,
            },
            Token {
                kind: Comma,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 3,
            },
            Token {
                kind: Colon,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 8,
            },
            Token {
                kind: RParen,
                len: 1,
            },
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: WS,
                len: 4,
            },
            Token {
                kind: Or,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 6,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 3,
            },
            Token {
                kind: Dot,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 2,
            },
            Token {
                kind: GT,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Literal {
                    kind: Int {
                        base: Dec,
                        empty_int: false,
                    },
                },
                len: 1,
            },
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: WS,
                len: 4,
            },
            Token {
                kind: Or,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 6,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 3,
            },
            Token {
                kind: Dot,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 2,
            },
            Token {
                kind: Eq,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 3,
            },
            Token {
                kind: Dot,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 2,
            },
            Token {
                kind: Plus,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 3,
            },
            Token {
                kind: Dot,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 2,
            },
            Token {
                kind: RBrace,
                len: 1,
            },
        ]
    "#]],
    );
}

#[test]
fn tokenize_program_with_comments_and_raw_ident() {
    let src =
        "// entity health\nprop `Health!` {\n    current: u32; /* current HP */\n    max: u32;\n}";
    check(
        src,
        expect![[r#"
        [
            Token {
                kind: LineComment,
                len: 16,
            },
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 4,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: RawIdent {
                    terminated: true,
                },
                len: 9,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: LBrace,
                len: 1,
            },
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: WS,
                len: 4,
            },
            Token {
                kind: Ident,
                len: 7,
            },
            Token {
                kind: Colon,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 3,
            },
            Token {
                kind: Semi,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: BlockComment {
                    terminated: true,
                },
                len: 16,
            },
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: WS,
                len: 4,
            },
            Token {
                kind: Ident,
                len: 3,
            },
            Token {
                kind: Colon,
                len: 1,
            },
            Token {
                kind: WS,
                len: 1,
            },
            Token {
                kind: Ident,
                len: 3,
            },
            Token {
                kind: Semi,
                len: 1,
            },
            Token {
                kind: NL,
                len: 1,
            },
            Token {
                kind: RBrace,
                len: 1,
            },
        ]
    "#]],
    );
}
