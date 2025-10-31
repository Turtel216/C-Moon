#ifndef LEXER_H_
#define LEXER_H_

#include <algorithm>
#include <exception>
#include <format>
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
  Token make_text();  // TODO
  std::optional<Token> skip_white_space();
  std::string make_int();
  std::string make_optional_exponent();
  Token make_eof_token(Position pos) const noexcept;

 public:
  Lexer(std::string input_str) : text(std::move(input_str)) {}

  Lexer() = delete;
};

class LexerError : public std::exception {
 private:
  Position pos;
  std::string msg;
  std::string formatted_msg;

 public:
  LexerError(std::string _msg, Position _pos) noexcept
      : msg(std::move(_msg)), pos(_pos) {
    formatted_msg =
        std::format("Line {}, column {}: {}", pos.line, pos.column, this->msg);
  }

  const char* what() const noexcept override;
};

#endif  // LEXER_H_
