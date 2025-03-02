#include <cassert>
#include <cstddef>
#include <cstdlib>
#include <vector>

#include "../../src/lexer/lexer.h"
#include "../../src/lexer/token.h"

int main(int argc, char *argv[]) {
  std::string test_string =
      "//some function \nint  main(void)\n {\n return 0;  \n}";
  std::vector<Token> expected_tokens{
      Token("int", TokenType::INT_KEYWORD),
      Token("main", TokenType::IDENTIFIER),
      Token("(", TokenType::OPEN_PARENTHESIS),
      Token("void", TokenType::VOID_KEYWORD),
      Token(")", TokenType::CLOSED_PARENTHESIS),
      Token("{", TokenType::OPEN_BRACE),
      Token("return", TokenType::RETURN_KEYWORD),
      Token("0", TokenType::CONSTANT),
      Token(";", TokenType::SEMICOLON),
      Token("}", TokenType::CLOSED_BRACE),
  };

  Lexer lex(test_string);
  cmoon::result<std::vector<Token>, cmoon::lexer_error> result = lex.start();

  assert(result.has_value());

  std::vector<Token> actual_tokens = result.value();

  assert(expected_tokens.size() == actual_tokens.size());

  for (size_t i = 0; i < actual_tokens.size(); ++i) {
    assert(expected_tokens[i] == actual_tokens[i]);
  }

  return EXIT_SUCCESS;
}
