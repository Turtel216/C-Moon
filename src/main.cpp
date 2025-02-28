#include <cstdlib>
#include <vector>

#include "lexer/token.h"
#include "parser/parser.h"

int main(int argc, char *argv[]) {
  // Example token stream for "int main(void) { return 42; }"
  std::vector<token> tokens = {
      token("int", INT_KEYWORD),       token("main", IDENTIFIER),
      token("(", OPEN_PARENTHESIS),    token("void", VOID_KEYWORD),
      token(")", CLOSED_PARENTHESIS),  token("{", OPEN_BRACE),
      token("return", RETURN_KEYWORD), token("42", CONSTANT),
      token(";", SEMICOLON),           token("}", CLOSED_BRACE)};

  Parser parser(tokens);
  bool success = parser.parse();

  return success ? 0 : 1;
}
