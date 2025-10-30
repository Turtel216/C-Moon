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
  std::optional<Token> skip_white_space();

  Token make_eof_token() const noexcept;

 public:
  Lexer(std::string&& input_str);

  Lexer() = delete;
};

#endif  // LEXER_H_
