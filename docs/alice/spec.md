# Alice Language Specification <!-- omit from toc -->

- [Introduction](#introduction)
- [1. Syntax and grammar](#1-syntax-and-grammar)
  - [1.1 Notation](#11-notation)
  - [1.2 Lexical grammar](#12-lexical-grammar)
    - [1.2.1 Whitespace and comments](#121-whitespace-and-comments)
    - [1.2.2 Keywords and operators](#122-keywords-and-operators)
    - [1.2.3 Literals](#123-literals)
    - [1.2.4 Identifiers](#124-identifiers)
    - [1.2.5 String mode grammar](#125-string-mode-grammar)
    - [1.2.6 Tokens](#126-tokens)
  - [1.3 Syntax grammar](#13-syntax-grammar)

## Introduction

This is the reference manual for the Alice programming language.

Alice is a domain specific language (a.k.a DSL) designed for game and realtime simulations development. It is based on the Entity-Component-System (a.k.a ECS) design pattern. Programs are constructed from:

1. **Ids** (Entities in ECS terms) - opaque identifiers that can be referenced or compared on equality.
2. **Props** (Components in ECS terms) - plain data structures whose values can be associated with an Id.
3. **Systems** - user logic containers that can query props and mutate their values.

The core idea behind Alice is that restricting developers to only these three ECS primitives naturally leads to software with the following characteristics out of the box:

1. **Performance** — cache-friendly data layout and a concurrency-friendly execution model.
2. **Maintainability** — composition over inheritance makes code easy to extend and refactor.
3. **Simplicity** — no complex abstractions; the mental model stays small and approachable.

## 1. Syntax and grammar

### 1.1 Notation

The syntax is specified using a [variant](https://en.wikipedia.org/wiki/Wirth_syntax_notation) of Extended Backus-Naur Form (EBNF):

```ebnf
syntax      = { production } .
production  = ProductionName "=" [ expression ] "." .
expression  = term { "|" term } .
term        = factor { factor } .
factor      = ProductionName | Token [ "…" Token ] | group | option | repetition .
group       = "(" expression ")" .
option      = "[" expression "]" .
repetition  = "{" expression "}" .
```

Productions are expressions constructed from terms and the following operators, in increasing precedence:

```text
|   alternation
()  grouping
[]  option (0 or 1 times)
{}  repetition (0 to n times)
```

CamelCase production names are used to identify lexical (terminal) tokens. Non-terminals are in snake_case. Lexical tokens are enclosed in double quotes "" or back quotes ``.

The form `a … b` represents the set of characters from a through b as alternatives. The horizontal ellipsis `…` is also used elsewhere in the spec to informally denote various enumerations or code snippets that are not further specified. The character `…` is not a token of the Alice language.

### 1.2 Lexical grammar

#### 1.2.1 Whitespace and comments

```ebnf
LineFeed       = '\n' .
CarriageReturn = '\r' .
NewLine        = LineFeed | (CarriageReturn [LineFeed]) .
BlockComment   = "/*" { BlockComment | <any character> } "*/" .
LineComment    = "//" { <any character except CarriageReturn and LineFeed> } .
Whitespace     = ( " " | "\t" | "\f" ) { ( " " | "\t" | "\f" ) } .
```

#### 1.2.2 Keywords and operators

```ebnf
Eq       = "=" .
Lt       = "<" .
Gt       = ">" .
Excl     = "!" .
Plus     = "+" .
Minus    = "-" .
Star     = "*" .
Slash    = "/" .
Dot      = "." .
Comma    = "," .
Semi     = ";" .
Colon    = ":" .
Quest    = "?" .
Pipe     = "|" .
LParen   = "(" .
RParen   = ")" .
LBrace   = "{" .
RBrace   = "}" .

EqEq       = "==" .
Ne         = "!=" .
Le         = "<=" .
Ge         = ">=" .
PlusEq     = "+=" .
MinusEq    = "-=" .
StarEq     = "*=" .
SlashEq    = "/=" .
ColonColon = "::" .

Id        = "id" .
Prop      = "prop" .
Sys       = "sys" .
Query     = "query" .
Namespace = "namespace" .
Const     = "const" .
And       = "and" .
Or        = "or" .
Using     = "using" .
Not       = "not" .
Mut       = "mut" .
If        = "if" .
Else      = "else" .
Match     = "match" .
Select    = "select" .
Filter    = "filter" .
Derive    = "derive" .
Insert    = "insert" .
Spawn     = "spawn" .
Despawn   = "despawn" .
```

#### 1.2.3 Literals

An integer literal is a sequence of digits representing an integer constant. An optional prefix sets a non-decimal base: `0b` or `0B` for binary, `0`, `0o`, or `0O` for octal, and `0x` or `0X` for hexadecimal. A single `0` is considered a decimal zero. In hexadecimal literals, letters `a` through `f` and `A` through `F` represent values `10` through `15`.

For readability, an underscore character `_` may appear after a base prefix or between successive digits; such underscores do not change the literal's value.

```ebnf
DecDigit = "0" … "9" .
BinDigit = "0" | "1" .
OctDigit = "0" … "7" .
HexDigit = "0" … "9" | "A" … "F" | "a" … "f" .

DecDigits = DecDigit { [ "_" ] DecDigit } .
BinDigits = BinDigit { [ "_" ] BinDigit } .
OctDigits = OctDigit { [ "_" ] OctDigit } .
HexDigits = HexDigit { [ "_" ] HexDigit } .

DecLit = "0" | ("1" … "9") [ [ "_" ] DecDigits ] .
BinLit = "0" ( "b" | "B" ) [ "_" ] BinDigits .
OctLit = "0" [ "o" | "O" ] [ "_" ] OctDigits .
HexLit = "0" ( "x" | "X" ) [ "_" ] HexDigits .

IntLit = DecLit | BinLit | OctLit | HexLit .

DecExponent  = ( "e" | "E" ) [ "+" | "-" ] DecDigits .
DecFloatLit = DecDigits "." [ DecDigits ] [ DecExponent ]
              | DecDigits DecExponent
              | "." DecDigits [ DecExponent ] .

HexExponent  = ( "p" | "P" ) [ "+" | "-" ] DecDigits .
HexMantissa  = [ "_" ] HexDigits "." [ HexDigits ]
              | [ "_" ] HexDigits
              | "." HexDigits .
HexFloatLit = "0" ( "x" | "X" ) HexMantissa HexExponent .

FloatLit = DecFloatLit | HexFloatLit .

BoolLit = "true" | "false" .
```

#### 1.2.4 Identifiers

```ebnf
UnicodeLetter = /* <any unicode character of categories Lu, Ll, Lt, Lm or Lo> */ .
UnicodeDigit = /* <any unicode character of category Nd> */ .
Ident = ( UnicodeLetter | "_" ) { UnicodeLetter | "_" | UnicodeDigit } .
```

#### 1.2.5 String mode grammar

TBD

#### 1.2.6 Tokens

These are all the valid tokens in one rule. Note that syntax grammar ignores tokens `BlockComment`, `LineComment` and `Whitespace`.

```ebnf
AliceToken = BlockComment
           | LineComment
           | Whitespace
           | NewLine
           | Eq
           | Lt
           | Gt
           | Excl
           | Plus
           | Minus
           | Star
           | Slash
           | Dot
           | Comma
           | Semi
           | Colon
           | Quest
           | Pipe
           | LParen
           | RParen
           | LBrace
           | RBrace
           | EqEq
           | Ne
           | Le
           | Ge
           | PlusEq
           | MinusEq
           | StarEq
           | SlashEq
           | ColonColon
           | Id
           | Prop
           | Sys
           | Query
           | Namespace
           | Const
           | And
           | Or
           | Using
           | Not
           | Mut
           | If
           | Else
           | Match
           | Select
           | Filter
           | Derive
           | Insert
           | Spawn
           | Despawn
           | IntLit
           | FloatLit
           | BoolLit
           | Ident .

EndOfInput = /* End of input */ .
```

### 1.3 Syntax grammar

The grammar below replaces some lexical grammar rules with explicit literals (where such replacement in trivial and always correct, for example, for keywords) for better readability.

```ebnf
alice_file = { top_level_stmt [semis] } EndOfInput .

top_level_stmt = using_stmt
               | decl .

using_stmt = "using" ident_path .

decl = prop_decl
     | sys_decl
     | const_decl
     | namespace_decl .

prop_decl      = "prop" ( unit_prop_decl | enum_prop_decl | record_prop_decl | tuple_prop_decl ) .

unit_prop_decl = Ident .

enum_prop_decl = Ident "=" enum_ctors .
enum_ctors     = enum_ctor { "|" enum_ctor } .
enum_ctor      = Ident [ "(" ident_path { "," ident_path } ")" ] .

record_prop_decl = Ident "{" field_decls "}" .
field_decls      = field_decl { "," field_decl } [ "," ] .
field_decl       = Ident ":" ident_path .

tuple_prop_decl = Ident "(" ident_path { "," ident_path } ")" .

sys_decl = "sys" sys_body .
sys_body = "{" { stage [semis] } "}" .

stage = select_stage
      | filter_stage
      | derive_stage
      | insert_stage
      | erase_stage
      | spawn_stage
      | despawn_stage .

select_stage    = "select" select_bindings .
select_bindings = select_binding { "," select_binding } .
select_binding  = [ Ident ":" ] ["mut"] ["?"] ident_path .

filter_stage = "filter" expr .

derive_stage = "derive" .

insert_stage = "insert" .

erase_stage = "erase" .

spawn_stage = "spawn" .

despawn_stage = "despawn" .

const_decl = "const" Ident "=" expr .

namespace_decl  = "namespace" ident_path namespace_scope .
namespace_scope = "{" { top_level_stmt [semis] } "}" .

ident_path = ["::"] Ident { "::" Ident } .

semis = ";" | NewLine { ";" | NewLine } .
```
