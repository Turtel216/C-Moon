#ifndef LEXER_H_
#define LEXER_H_

#include <optional>
#include <string>

#include "token.h"

class Lexer {
 private:
  std::string text;
  int pos;
  char c_char;
  int line = 1, colunm = 0;

  char advance();
  char peek() const;
  Token next_token();
  Token make_number();
  std::optional<Token> skip_white_space();
  std::string make_int();                // TODO
  std::string make_optional_exponent();  // TODO
  Token make_eof_token(Position pos) const noexcept;
  Token make_error_token(const std::string msg, Position pos) const noexcept;

 public:
  Lexer(std::string&& input_str);

  Lexer() = delete;
};

#endif  // LEXER_H_
