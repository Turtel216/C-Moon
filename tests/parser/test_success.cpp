#include <cassert>
#include <cstdlib>
#include <vector>

#include "../../src/lexer/token.h"
#include "../../src/parser/parser.h"

int main(int argc, char *argv[]) {
  // Example token stream for "int main(void) { return 42; }"
  std::vector<token> tokens = {
      token("int", INT_KEYWORD),       token("main", IDENTIFIER),
      token("(", OPEN_PARENTHESIS),    token("void", VOID_KEYWORD),
      token(")", CLOSED_PARENTHESIS),  token("{", OPEN_BRACE),
      token("return", RETURN_KEYWORD), token("42", CONSTANT),
      token(";", SEMICOLON),           token("}", CLOSED_BRACE)};

  Parser parser(tokens);
  auto result = parser.parse();

  // TODO: check correctness of AST tree
  assert(result.has_value());

  return 0;
}
