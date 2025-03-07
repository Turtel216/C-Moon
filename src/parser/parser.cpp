#include "parser.h"

#include <memory>
#include <optional>
#include <string>

#include "ast.h"

// TODO: Add proper values to ast::node constructors. The string value should be
// the actual value.
// Also make the unique pointers towards the end of the function to safe time
// when a function will throw an exception anyways

// Get the current token
auto Parser::current_token() const -> const Token& {
  if (current_position >= tokens.size()) {
    throw cmoon::ParseException("Unexpected end of input");
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
auto Parser::match(TokenType expected_type) -> std::optional<Token> {
  if (current_position >= tokens.size()) {
    return std::nullopt;
  }  // if

  if (tokens[current_position].type == expected_type) {
    advance();
    return tokens[current_position -
                  1];  // TODO: dont like that -1. Try and avoid it.
  }  // if

  return std::nullopt;
}  // match

// Expect a token of a specific type (throw error if not found)
auto Parser::expect(TokenType expected_type, const std::string& error_message)
    -> void {
  if (!match(expected_type).has_value()) {
    throw cmoon::ParseException(error_message);
  }  // if
}  // expect

// Expect a token of a specific type (throw error if not found) and return the
// token.
[[nodiscard]] auto Parser::expect_and_rtn(TokenType expected_type,
                                          const std::string& error_message)
    -> Token {
  auto token = match(expected_type);
  if (!token.has_value()) {
    throw cmoon::ParseException(error_message);
  }  // if

  return token.value();
}  // expect_and_rtn

// Parsing functions for each non-terminal in the grammar

// <program> ::= <function>
auto Parser::parse_program() -> std::unique_ptr<ast::Node> {
  auto result = std::make_unique<ast::Node>("Program", ast::Type::PROGRAM);
  result->next = parse_function();

  // Ensure we've consumed all tokens
  if (current_position < tokens.size()) {
    throw cmoon::ParseException("Unexpected tokens after end of program");
  }  // if

  return result;
}  // parse_program

// <function> ::= "int" <identifier> "(" "void" ")" "{" <statement> "}"
auto Parser::parse_function() -> std::unique_ptr<ast::Node> {
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

  // Create node
  auto result = std::make_unique<ast::Node>("Int", ast::Type::CONSTANT);
  // Parse statement
  // Point to next node in AST tree
  result->next = parse_statement();

  // Match "}"
  expect(CLOSED_BRACE, "Expected '}' to end function body");

  return result;
}  // parse_function

// <statement> ::= "return" <exp> ";"
auto Parser::parse_statement() -> std::unique_ptr<ast::Node> {
  // Match "return"
  expect(RETURN_KEYWORD, "Expected 'return' keyword");

  // Create node
  auto result = std::make_unique<ast::Node>("Return", ast::Type::RETURN);
  // Parse expression
  // Point to next node in AST tree
  result->next = parse_exp();

  // Match ";"
  expect(SEMICOLON, "Expected ';' after return statement");

  return result;
}  // parse_statement

// <exp> ::= <int>
auto Parser::parse_exp() -> std::unique_ptr<ast::Node> {
  // Match constant
  expect(CONSTANT, "Expected integer constant in expression");

  // Create node
  auto result = std::make_unique<ast::Node>("Return", ast::Type::RETURN);
  // TODO: Point to next node in AST tree. Which is null

  return result;
}  // parse_exp

// Parse the input and return success/failure
[[nodiscard]] auto Parser::parse()
    -> cmoon::result<std::unique_ptr<ast::Node>, cmoon::ParserError> {
  try {
    return parse_program();
  }  // try
  catch (const cmoon::ParseException& e) {
    std::string exc_msg = e.what();
    std::string error_msg = "Parse error: " + exc_msg + "\n";
    if (current_position < tokens.size()) {
      error_msg += "At token: " + tokens[current_position].lexeme;
    }  // if
    else {
      error_msg = "At end of input";
    }  // else
    return cmoon::ParserError(error_msg);
  }  // catch
}  // parse
