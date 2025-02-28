#include "parser.h"

#include <iostream>

// Get the current token
auto Parser::current_token() const -> const token& {
  if (current_position >= tokens.size()) {
    throw cmoon::ParseError("Unexpected end of input");
  }  // if
  return tokens[current_position];
}  // current_token

// Advance to the next token
auto Parser::advance() -> void {
  if (current_position < tokens.size()) {
    current_position++;
  }  // if
}  // advance

// Check if the current token matches the expected type
auto Parser::match(TokenType expected_type) -> bool {
  if (current_position >= tokens.size()) {
    return false;
  }  // if

  if (tokens[current_position].type == expected_type) {
    advance();
    return true;
  }  // if

  return false;
}  // match

// Expect a token of a specific type (throw error if not found)
auto Parser::expect(TokenType expected_type, const std::string& error_message)
    -> void {
  if (!match(expected_type)) {
    throw cmoon::ParseError(error_message);
  }  // if
}  // expect

// Parsing functions for each non-terminal in the grammar

// <program> ::= <function>
auto Parser::parse_program() -> void {
  std::cout << "Parsing program..." << std::endl;
  parse_function();

  // Ensure we've consumed all tokens
  if (current_position < tokens.size()) {
    throw cmoon::ParseError("Unexpected tokens after end of program");
  }  // if
}  // parse_program

// <function> ::= "int" <identifier> "(" "void" ")" "{" <statement> "}"
auto Parser::parse_function() -> void {
  std::cout << "Parsing function..." << std::endl;

  // Match "int"
  expect(INT_KEYWORD, "Expected 'int' keyword at start of function");

  // Match identifier
  expect(IDENTIFIER, "Expected function identifier");

  // Match "("
  expect(OPEN_PARENTHESIS, "Expected '(' after function identifier");

  // Match "void"
  expect(VOID_KEYWORD, "Expected 'void' keyword in function parameters");

  // Match ")"
  expect(CLOSED_PARENTHESIS, "Expected ')' after function parameters");

  // Match "{"
  expect(OPEN_BRACE, "Expected '{' to begin function body");

  // Parse statement
  parse_statement();

  // Match "}"
  expect(CLOSED_BRACE, "Expected '}' to end function body");
}  // parse_function

// <statement> ::= "return" <exp> ";"
auto Parser::parse_statement() -> void {
  std::cout << "Parsing statement..." << std::endl;

  // Match "return"
  expect(RETURN_KEYWORD, "Expected 'return' keyword");

  // Parse expression
  parse_exp();

  // Match ";"
  expect(SEMICOLON, "Expected ';' after return statement");
}  // parse_statement

// <exp> ::= <int>
auto Parser::parse_exp() -> void {
  std::cout << "Parsing expression..." << std::endl;

  // Match constant
  expect(CONSTANT, "Expected integer constant in expression");
}  // parse_exp

// Parse the input and return success/failure
auto Parser::parse() -> bool {
  try {
    parse_program();
    std::cout << "Parsing completed successfully!" << std::endl;
    return true;
  }  // try
  catch (const cmoon::ParseError& e) {
    std::cerr << "Parse error: " << e.what() << std::endl;
    if (current_position < tokens.size()) {
      std::cerr << "At token: " << tokens[current_position].lexeme << std::endl;
    }  // if
    else {
      std::cerr << "At end of input" << std::endl;
    }  // else
    return false;
  }  // catch
}  // parse
