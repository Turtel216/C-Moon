#include <cassert>
#include <cstddef>
#include <cstdlib>
#include <vector>

#include "../../src/lexer/lexer.h"
#include "../../src/lexer/token.h"

int main(int argc, char *argv[]) {
  std::string test_string =
      "//some function \nint  main(void)\n {\n return 0;  \n}";
  std::vector<token> expected_tokens{
      token("int", TokenType::INT_KEYWORD),
      token("main", TokenType::IDENTIFIER),
      token("(", TokenType::OPEN_PARENTHESIS),
      token("void", TokenType::VOID_KEYWORD),
      token(")", TokenType::CLOSED_PARENTHESIS),
      token("{", TokenType::OPEN_BRACE),
      token("return", TokenType::RETURN_KEYWORD),
      token("0", TokenType::CONSTANT),
      token(";", TokenType::SEMICOLON),
      token("}", TokenType::CLOSED_BRACE),
  };

  lexer lex(test_string);
  cmoon::result<std::vector<token>, cmoon::lexer_error> result = lex.start();

  assert(result.has_value());

  std::vector<token> actual_tokens = result.value();

  assert(expected_tokens.size() == actual_tokens.size());

  for (size_t i = 0; i < actual_tokens.size(); ++i) {
    assert(expected_tokens[i] == actual_tokens[i]);
  }

  return EXIT_SUCCESS;
}
