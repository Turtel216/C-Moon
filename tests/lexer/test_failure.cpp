#include <cassert>
#include <cstdlib>
#include <vector>

#include "../../src/lexer/lexer.h"
#include "../../src/lexer/token.h"

int main(int argc, char *argv[]) {
  std::string test_string = "int main(void) #";
  Lexer lex(test_string);

  cmoon::result<std::vector<Token>, cmoon::lexer_error> result = lex.start();
  cmoon::lexer_error expected_error =
      cmoon::lexer_error("uncrecognized character");

  assert(!result.has_value());

  cmoon::lexer_error actual_error = result.error();
  assert(actual_error == expected_error);

  return EXIT_SUCCESS;
}
