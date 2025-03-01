#ifndef PARSER_H_
#define PARSER_H_

#include <memory>
#include <optional>
#include <vector>

#include "../lexer/token.h"
#include "../utils/errors/errors.h"
#include "ast.h"

// Recursive Descent Parser for C-Moon
class Parser {
 private:
  std::vector<token> tokens;  // Token stream from lexer
  size_t current_position;    // Current position in token stream

  // Get the current token
  const token& current_token() const;

  // Advance to the next token
  auto advance() -> void;

  // Check if the current token matches the expected type
  auto match(TokenType expected_type) -> bool;

  // Expect a token of a specific type (throw error if not found)
  auto expect(TokenType expected_type, const std::string& error_message)
      -> void;

  // Parsing functions for each non-terminal in the grammar

  // <program> ::= <function>
  auto parse_program() -> std::unique_ptr<ast::node>;

  // <function> ::= "int" <identifier> "(" "void" ")" "{" <statement> "}"
  auto parse_function() -> std::unique_ptr<ast::node>;

  // <statement> ::= "return" <exp> ";"
  auto parse_statement() -> std::unique_ptr<ast::node>;

  // <exp> ::= <int>
  auto parse_exp() -> std::unique_ptr<ast::node>;

 public:
  // Constructor
  explicit Parser(const std::vector<token>& tokens)
      : tokens(tokens), current_position(0) {}

  // Parse the input and return an optional for success/failure
  auto parse() -> std::optional<std::unique_ptr<ast::node>>;
};  // Parser

// Example usage
/*int main() {
}*/

#endif  // PARSER_H_
