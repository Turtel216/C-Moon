#include "lexer.h"

#include <cctype>
#include <cstddef>
#include <map>
#include <optional>
#include <string>

#include "token.h"

// key-value map of C-Moon keywords
static const std::map<std::string, Token> keywords{
    {"int", Token("int", TokenType::INT_KEYWORD)},
    {"void", Token("void", TokenType::VOID_KEYWORD)},
    {"return", Token("return", TokenType::RETURN_KEYWORD)}};

// Start the lexing process. Returns either a vector of the tokenized input
// string or an lexer_rror
[[nodiscard]] auto Lexer::start() noexcept
    -> cmoon::result<std::vector<Token>, cmoon::lexer_error> {
  while (!is_at_end()) {
    std::optional<Token> token_opt = next_token();
    if (token_opt.has_value()) {
      tokens.push_back(token_opt.value());
      advance();
    }  // if
    else {
      std::string error_msg =
          "uncrecognized character";  // TODO: add the missing character to the
                                      // string
      return cmoon::lexer_error(error_msg);
    }  // else
  }  // while

  return tokens;
}  // start

// Returns the next recognized token.
[[nodiscard]] auto Lexer::next_token() noexcept -> std::optional<Token> {
  skip_whitespace();

  char curr_char = peek();

  switch (curr_char) {
    case ';':
      return Token(";", TokenType::SEMICOLON);
      break;
    case '(':
      return Token("(", TokenType::OPEN_PARENTHESIS);
      break;
    case ')':
      return Token(")", TokenType::CLOSED_PARENTHESIS);
      break;
    case '{':
      return Token("{", TokenType::OPEN_BRACE);
      break;
    case '}':
      return Token("}", TokenType::CLOSED_BRACE);
      break;
    default:
      if (std::isalnum(curr_char)) {
        if (std::isdigit(curr_char)) {
          return lex_number();
          break;
        }  // if

        return lex_identifier();
        break;
      }  // if

      return std::nullopt;
  }  // switch
}  // next_token

// Tokenize a number
[[nodiscard]] auto Lexer::lex_number() noexcept -> Token {
  lexeme_start = pos;
  size_t sub_len = 1;
  while (std::isdigit(peek_next())) {
    ++sub_len;
    advance();
  }  // while

  std::string lexeme = input.substr(lexeme_start, sub_len);
  return Token(lexeme, TokenType::CONSTANT);
}  // lex_number

// Tokenize an identifier
[[nodiscard]] auto Lexer::lex_identifier() noexcept -> Token {
  lexeme_start = pos;
  size_t sub_len = 1;
  while (std::isalpha(peek_next())) {
    sub_len++;
    advance();
  }  // while

  // Get keyword/identifier string
  std::string lexeme = input.substr(lexeme_start, sub_len);

  // Check if lexeme is a keyword
  std::optional<Token> keyword_opt = match_keyword(lexeme);
  if (keyword_opt.has_value()) {
    return keyword_opt.value();
  }  // if

  // Not a keyword, return identifier
  return Token(lexeme, TokenType::IDENTIFIER);
}  // lex_identifier

// Check if the given string is a keyword. Return option of either the keyword
// token or an empty optional
[[nodiscard]] auto Lexer::match_keyword(std::string& str) noexcept
    -> std::optional<Token> const {
  if (auto result = keywords.find(str); result != keywords.end()) {
    return result->second;
  }  // if

  return std::nullopt;
}  // match_keyword

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
          }  // while
        }  // if
        else {
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
