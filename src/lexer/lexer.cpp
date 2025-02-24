#include "lexer.h"

#include <optional>

// Start the lexing process. Returns the tokenized input string.
auto start() noexcept -> std::vector<Token>;

// Moves the lexer and returns the next recognized token.
[[nodiscard]] auto next_token() noexcept -> Token;

// Tokenize a number
[[nodiscard]] auto lex_number() noexcept -> Token;

// Tokenize an identifier
[[nodiscard]] auto lex_identifier() noexcept -> Token;

// Check if the given string is a keyword. Return option of either the keyword
// token or an empty optional
[[nodiscard]] auto match_keyword(std::string& str) noexcept
    -> const std::optional<Token>;

// peek returns the current character specified by `pos`. Returns 0
// if the end of input string is reached.
auto Lexer::peek() noexcept -> char const {
  if (is_at_end()) {
    return '\0';
  }  // if

  return input.at(pos);
}  // peek

// peek_next returns the next character specified by `pos`. Returns 0
// if the end of input string is reached.
auto Lexer::peek_next() noexcept -> char const {
  if ((pos + 1) >= input.length()) {
    return '\0';
  }  // if

  return input.at(pos + 1);
}  // peek_next

// Check if the next character is the expected character
auto Lexer::check_next(char expected) noexcept -> bool const {
  return expected == peek_next();
}  // check_next

// Skips whitespace and comments, advancing the current character index
// appropriately. Handles single-line comments starting with '//'. Modifies
// `pos`
auto Lexer::skip_whitespace() noexcept -> void {
  while (!is_at_end()) {
    switch (peek()) {
      case ' ':
      case '\r':
      case '\t':
        advance();
        break;

      case '\n':
        advance();
        break;

      case '/':
        if (peek_next() == '/') {
          while (peek() != '\n' && !is_at_end()) {
            advance();
          }  // if
        } else {
          return;
        }  // else
        break;

      default:
        return;
    }  // switch
  }  // while
}  // skip_whitespace

// Returns true if the lexer reached the end of the input string.
// Returns false otherwise
auto inline __attribute__((always_inline)) Lexer::is_at_end() noexcept
    -> bool const {
  return pos >= input.length();
}  // is_at_end

// Advance the lexer index by 1. The method does not check for
// eof.
auto inline __attribute__((always_inline)) Lexer::advance() noexcept -> void {
  pos++;
}  // advance
