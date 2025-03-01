#include <cassert>
#include <cstdlib>
#include <vector>

#include "../../src/lexer/token.h"
#include "../../src/parser/parser.h"

int main(int argc, char *argv[]) {
  // Example token stream for "main(void) { return 42; }. Should return nullopt"
  std::vector<token> tokens_missing_int = {
      token("main", IDENTIFIER),   token("(", OPEN_PARENTHESIS),
      token("void", VOID_KEYWORD), token(")", CLOSED_PARENTHESIS),
      token("{", OPEN_BRACE),      token("return", RETURN_KEYWORD),
      token("42", CONSTANT),       token(";", SEMICOLON),
      token("}", CLOSED_BRACE)};

  Parser parser_missing_int(tokens_missing_int);
  auto result_missing_int = parser_missing_int.parse();

  assert(!result_missing_int.has_value());

  // Example token stream for "int main void) { return 42; }. Should return
  // nullopt"
  std::vector<token> tokens_missing_parenthesis = {
      token("int", INT_KEYWORD),   token("main", IDENTIFIER),
      token("void", VOID_KEYWORD), token(")", CLOSED_PARENTHESIS),
      token("{", OPEN_BRACE),      token("return", RETURN_KEYWORD),
      token("42", CONSTANT),       token(";", SEMICOLON),
      token("}", CLOSED_BRACE)};

  Parser parser_missing_parenthesis(tokens_missing_parenthesis);
  auto result_missing_parenthesis = parser_missing_parenthesis.parse();

  assert(!result_missing_parenthesis.has_value());

  // Example token stream for "int main(void) { return 42 }. Should return
  // nullopt"
  std::vector<token> tokens_missing_semicolone = {
      token("int", INT_KEYWORD),       token("main", IDENTIFIER),
      token("(", OPEN_PARENTHESIS),    token("void", VOID_KEYWORD),
      token(")", CLOSED_PARENTHESIS),  token("{", OPEN_BRACE),
      token("return", RETURN_KEYWORD), token("42", CONSTANT),
      token("}", CLOSED_BRACE)};

  Parser parser_missing_semicolone(tokens_missing_semicolone);
  auto result_missing_semiclone = parser_missing_semicolone.parse();

  assert(!result_missing_semiclone.has_value());

  // Example token stream for "int main(void) { return 42 }. Should return
  // nullopt"
  std::vector<token> tokens_missing_constant = {
      token("int", INT_KEYWORD),       token("main", IDENTIFIER),
      token("(", OPEN_PARENTHESIS),    token("void", VOID_KEYWORD),
      token(")", CLOSED_PARENTHESIS),  token("{", OPEN_BRACE),
      token("return", RETURN_KEYWORD), token(";", SEMICOLON),
      token("}", CLOSED_BRACE)};

  Parser parser_missing_constant(tokens_missing_constant);
  auto result_missing_constant = parser_missing_constant.parse();

  assert(!result_missing_constant.has_value());
  return 0;
}
