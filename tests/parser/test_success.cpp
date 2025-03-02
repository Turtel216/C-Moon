#include <cassert>
#include <cstdlib>
#include <vector>

#include "../../src/lexer/token.h"
#include "../../src/parser/parser.h"

int main(int argc, char *argv[]) {
  // Example token stream for "int main(void) { return 42; }"
  std::vector<Token> tokens = {
      Token("int", INT_KEYWORD),       Token("main", IDENTIFIER),
      Token("(", OPEN_PARENTHESIS),    Token("void", VOID_KEYWORD),
      Token(")", CLOSED_PARENTHESIS),  Token("{", OPEN_BRACE),
      Token("return", RETURN_KEYWORD), Token("42", CONSTANT),
      Token(";", SEMICOLON),           Token("}", CLOSED_BRACE)};

  Parser parser(tokens);
  auto result = parser.parse();

  // TODO: check correctness of AST tree
  assert(result.has_value());

  return 0;
}
