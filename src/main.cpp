#include <cstdlib>
#include <vector>

#include "lexer/token.h"
#include "parser/parser.h"

int main(int argc, char *argv[]) {
  // Example Token stream for "int main(void) { return 42; }"
  std::vector<Token> tokens = {
      Token("int", INT_KEYWORD),       Token("main", IDENTIFIER),
      Token("(", OPEN_PARENTHESIS),    Token("void", VOID_KEYWORD),
      Token(")", CLOSED_PARENTHESIS),  Token("{", OPEN_BRACE),
      Token("return", RETURN_KEYWORD), Token("42", CONSTANT),
      Token(";", SEMICOLON),           Token("}", CLOSED_BRACE)};

  Parser parser(tokens);
  auto result = parser.parse();

  return result.has_value() ? 0 : 1;
}
