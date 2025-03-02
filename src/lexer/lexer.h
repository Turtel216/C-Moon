#ifndef LEXER_H_
#define LEXER_H_

#include <cstddef>
#include <optional>
#include <string>
#include <vector>

#include "../utils/errors/errors.h"
#include "../utils/result.h"
#include "token.h"

// The Lexer generates an array of tokens from a given input string
class Lexer {
 public:
  // Input string that should be tokenized
  const std::string& input;

  // Initialises the lexer
  Lexer(const std::string& input)
      : input(input), pos(0), lexeme_start(0) {}  // Constructor

  Lexer() = delete;  // Default constructor

  // Start the lexing process. Returns either a vector of the tokenized input
  // string or an lexer_rror
  [[nodiscard]] auto start() noexcept
      -> cmoon::result<std::vector<Token>, cmoon::LexerError>;

 private:
  // Current position(index) of the lexer.
  size_t pos;
  // Start of the current lexeme
  size_t lexeme_start;
  // Vector holding the tokens generated by the lexer
  std::vector<Token> tokens;

  // Moves the lexer and returns the next recognized token.
  [[nodiscard]] auto next_token() noexcept -> std::optional<Token>;

  // Tokenize a number
  [[nodiscard]] auto lex_number() noexcept -> Token;

  // Tokenize an identifier
  [[nodiscard]] auto lex_identifier() noexcept -> Token;

  // Check if the given string is a keyword. Return option of either the keyword
  // token or an empty optional
  [[nodiscard]] auto match_keyword(std::string& str) noexcept
      -> std::optional<Token> const;

  // peek returns the current character specified by `pos`. Returns 0
  // if the end of input string is reached.
  auto peek() noexcept -> char const;

  // peek_next returns the next character specified by `pos`. Returns 0
  // if the end of input string is reached.
  auto peek_next() noexcept -> char const;

  // Check if the next character is the expected character
  auto check_next(char expected) noexcept -> bool const;

  // Skips whitespace and comments, advancing the current character index
  // appropriately. Handles single-line comments starting with '//'. Modifies
  // `pos`
  auto skip_whitespace() noexcept -> void;

  // Advance the lexer index by 1. The method does not check for
  // eof.
  auto inline __attribute__((always_inline)) advance() noexcept -> void;

  // Returns true if the lexer reached the end of the input string.
  // Returns false otherwise
  auto inline __attribute__((always_inline)) is_at_end() noexcept -> bool const;
};  // Lexer

#endif  // LEXER_H_
