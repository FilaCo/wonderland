#ifndef ALICE_AST_TOKEN_HPP
#define ALICE_AST_TOKEN_HPP

enum class TokenKind : unsigned char {
#define TOKEN(ID) ID
#include "Alice/AST/Token.def"
};

struct Token {
  TokenKind Kind;
};

#endif // ALICE_AST_TOKEN_HPP