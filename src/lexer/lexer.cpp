#include "lexer.h"

#include <cctype>
#include <optional>
#include <string>

#include "token.h"

char Lexer::advance() {
  if (pos < text.length())
    c_char = text.at(pos);
  else
    c_char = 0;  // No More characters

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

  if (std::isdigit(c_char) || c_char == '.') {
    return make_number();
  }

  if (std::isalpha(c_char) || c_char == '_') {
    return make_text();
  }

  // Letter
  // Symbol

  return make_eof_token({line, colunm});
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
        return make_eof_token({line, colunm});
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

        if (c_char == 0)
          return Token("Unclosed Comment", TokenType::ERROR, {line, colunm});

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

Token Lexer::make_number() {
  std::string value = "";
  bool parsing_double = c_char == '.';

  // loeading dot float
  if (c_char == '.') {
    value += c_char;
    advance();
  }

  value += make_int();
  if (value == ".") {
    throw LexerError("Illegal floating point constant", {line, colunm});
  }

  bool long_const = false;
  bool unsigned_const = false;
  char prev_char = c_char;
  if (value.length() > 0 && !parsing_double) {
    if (c_char == 'L' || c_char == 'l') {
      // long constant
      advance();
      long_const = true;

      if (c_char == 'U' || c_char == 'u') {
        // unsigned constant
        advance();
        unsigned_const = true;
      }
    } else if (c_char == 'U' || c_char == 'u') {
      // unsigned constant
      advance();
      unsigned_const = true;

      if (c_char == 'L' || c_char == 'l') {
        // long constant
        advance();
        long_const = true;
      }
    }
  }

  if ((c_char == '.' || c_char == 'E' || c_char == 'e') &&
      (prev_char == 'L' || prev_char == 'l' || prev_char == 'U' ||
       prev_char == 'u')) {
    throw LexerError(
        "Illegal floating point constant" + value + prev_char + '.',
        {line, colunm});
  }

  if (c_char == '.') {
    value += c_char;
    advance();
    parsing_double = true;

    value += make_int();
  }

  parsing_double |= (c_char == 'E' || c_char == 'e');

  if (parsing_double) {
    value += make_optional_exponent();
  } else if (std::isalpha(c_char)) {
    throw LexerError("Illgegal floating point constant", {line, colunm});
  }

  if (parsing_double) {
    return Token(value, TokenType::NUMERIC_LITERAL, {line, colunm},
                 VarType::DOUBLE);
  }

  if (parsing_double && long_const) {
    return Token(value, TokenType::NUMERIC_LITERAL, {line, colunm},
                 VarType::UNSIGNED_LONG);
  }

  if (long_const) {
    return Token(value, TokenType::NUMERIC_LITERAL, {line, colunm},
                 VarType::LONG);
  }

  if (unsigned_const) {
    return Token(value, TokenType::NUMERIC_LITERAL, {line, colunm},
                 VarType::UNSIGNED_INT);
  }

  // Int
  return Token(value, TokenType::NUMERIC_LITERAL, {line, colunm}, VarType::INT);
}

std::string inline Lexer::make_int() {
  std::string sb = "";

  if (c_char == '_') {
    throw LexerError("Cannot start number with underscore", {line, colunm});
  }

  while (std::isdigit(c_char) || c_char) {
    sb += c_char;
    advance();
  }

  return sb;
}

std::string inline Lexer::make_optional_exponent() {
  std::string value = "";

  if (c_char == 'E' || c_char == 'e') {
    value += c_char;
    advance();

    if (c_char == '+' || c_char == '-') {
      value += c_char;
      advance();
    }

    std::string exp = make_int();
    if (exp.length() == 0) {
      throw LexerError("Invalid floating point constant" + value,
                       {line, colunm});
    }

    value += exp;
  }

  if (c_char == '.' || std::isalpha(c_char)) {
    throw "Invalid character after floating point constant: " +
        std::to_string(c_char);
  }

  return value;
}

Token inline Lexer::make_eof_token(Position pos) const noexcept {
  return Token("", TokenType::EOF_TOKEN, pos);
}

//
// LexerError implementation
//

const char* LexerError::what() const noexcept { return formatted_msg.c_str(); }
