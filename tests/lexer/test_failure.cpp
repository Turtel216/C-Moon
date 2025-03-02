#include <cassert>
#include <cstdlib>
#include <vector>

#include "../../src/lexer/lexer.h"
#include "../../src/lexer/token.h"

int main(int argc, char *argv[]) {
  std::string test_string = "int main(void) #";
  Lexer lex(test_string);

  cmoon::result<std::vector<Token>, cmoon::LexerError> result = lex.start();
  cmoon::LexerError expected_error =
      cmoon::LexerError("uncrecognized character");

  assert(!result.has_value());

  cmoon::LexerError actual_error = result.error();
  assert(actual_error == expected_error);

  return EXIT_SUCCESS;
}
