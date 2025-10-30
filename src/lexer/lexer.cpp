#include "lexer.h"

#include <optional>

#include "token.h"

char Lexer::advance() {
  if (pos < text.length())
    c_char = text.at(pos);
  else
    c_char = 0;  // No More character

  pos++;
  colunm++;
  return c_char;
}

char Lexer::peek() const {
  if (pos < text.length()) {
    return text.at(pos);
  }

  // Signal end of file
  return 0;
}

Token Lexer::next_token() {
  std::optional<Token> maybeToken = skip_white_space();

  if (maybeToken.has_value()) return maybeToken.value();

  // Digit
  // Letter
  // Symbol

  return make_eof_token();
}

std::optional<Token> Lexer::skip_white_space() {
  while (true) {
    while (c_char == ' ' || c_char == '\n' || c_char == '\t' ||
           c_char == '\r') {
      if (c_char == '\n' || c_char == '\r') {  // New line
        line++;
        colunm = 0;
      }
      advance();
    }

    // None comment text, return empty
    if (c_char != '/') {
      return {};
    }

    char next = peek();
    // C style Comment
    if (next == '/') {
      advance();
      advance();

      // Advance until end of line or EOF is met
      while (c_char != 0 && c_char != '\n') {
        advance();
      }
      if (c_char == 0) {  // EOF
        return make_eof_token();
      }

      line++;
      colunm = 0;
      advance();  // skip \n
      continue;
    }

    // C++ Style Comment
    if (next == '*') {
      advance();
      advance();

      // Tracking if a closig character is found
      bool closing = false;

      while (closing) {
        while (c_char != 0 && c_char != '*') {
          advance();
        }

        if (c_char == 0) return Token("Unclosed Comment", TokenType::ERROR);

        advance();  // skip the star

        if (c_char == '/') {
          advance();  // skip the slash
          closing = true;
        }
      }
      continue;
    }
    return {};
  }
}

Token Lexer::make_eof_token() const noexcept {
  return Token("", TokenType::EOF_TOKEN);
}
