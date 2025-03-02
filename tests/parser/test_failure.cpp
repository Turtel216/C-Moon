#include <cassert>
#include <cstdlib>
#include <vector>

#include "../../src/lexer/token.h"
#include "../../src/parser/parser.h"

int main(int argc, char *argv[]) {
  // Example Token stream for "main(void) { return 42; }. Should return nullopt"
  std::vector<Token> tokens_missing_int = {
      Token("main", IDENTIFIER),   Token("(", OPEN_PARENTHESIS),
      Token("void", VOID_KEYWORD), Token(")", CLOSED_PARENTHESIS),
      Token("{", OPEN_BRACE),      Token("return", RETURN_KEYWORD),
      Token("42", CONSTANT),       Token(";", SEMICOLON),
      Token("}", CLOSED_BRACE)};

  Parser parser_missing_int(tokens_missing_int);
  auto result_missing_int = parser_missing_int.parse();

  assert(!result_missing_int.has_value());

  // Example Token stream for "int main void) { return 42; }. Should return
  // nullopt"
  std::vector<Token> tokens_missing_parenthesis = {
      Token("int", INT_KEYWORD),   Token("main", IDENTIFIER),
      Token("void", VOID_KEYWORD), Token(")", CLOSED_PARENTHESIS),
      Token("{", OPEN_BRACE),      Token("return", RETURN_KEYWORD),
      Token("42", CONSTANT),       Token(";", SEMICOLON),
      Token("}", CLOSED_BRACE)};

  Parser parser_missing_parenthesis(tokens_missing_parenthesis);
  auto result_missing_parenthesis = parser_missing_parenthesis.parse();

  assert(!result_missing_parenthesis.has_value());

  // Example Token stream for "int main(void) { return 42 }. Should return
  // nullopt"
  std::vector<Token> tokens_missing_semicolone = {
      Token("int", INT_KEYWORD),       Token("main", IDENTIFIER),
      Token("(", OPEN_PARENTHESIS),    Token("void", VOID_KEYWORD),
      Token(")", CLOSED_PARENTHESIS),  Token("{", OPEN_BRACE),
      Token("return", RETURN_KEYWORD), Token("42", CONSTANT),
      Token("}", CLOSED_BRACE)};

  Parser parser_missing_semicolone(tokens_missing_semicolone);
  auto result_missing_semiclone = parser_missing_semicolone.parse();

  assert(!result_missing_semiclone.has_value());

  // Example Token stream for "int main(void) { return 42 }. Should return
  // nullopt"
  std::vector<Token> tokens_missing_constant = {
      Token("int", INT_KEYWORD),       Token("main", IDENTIFIER),
      Token("(", OPEN_PARENTHESIS),    Token("void", VOID_KEYWORD),
      Token(")", CLOSED_PARENTHESIS),  Token("{", OPEN_BRACE),
      Token("return", RETURN_KEYWORD), Token(";", SEMICOLON),
      Token("}", CLOSED_BRACE)};

  Parser parser_missing_constant(tokens_missing_constant);
  auto result_missing_constant = parser_missing_constant.parse();

  assert(!result_missing_constant.has_value());
  return 0;
}
