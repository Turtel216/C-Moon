#ifndef PARSER_H_
#define PARSER_H_

#include <vector>

#include "../lexer/token.h"
#include "../utils/errors/errors.h"

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
  auto parse_program() -> void;

  // <function> ::= "int" <identifier> "(" "void" ")" "{" <statement> "}"
  auto parse_function() -> void;

  // <statement> ::= "return" <exp> ";"
  auto parse_statement() -> void;

  // <exp> ::= <int>
  auto parse_exp() -> void;

 public:
  // Constructor
  explicit Parser(const std::vector<token>& tokens)
      : tokens(tokens), current_position(0) {}

  // Parse the input and return success/failure
  auto parse() -> bool;
};  // Parser

// Example usage
/*int main() {
}*/

#endif  // PARSER_H_
