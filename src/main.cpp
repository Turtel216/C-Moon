#include <cstdlib>
#include <iostream>
#include <vector>

#include "./lexer/lexer.h"
#include "lexer/token.h"

int main(int argc, char *argv[]) {
  std::string input = "int main(void);";
  Lexer lexer = Lexer(input);
  std::vector<Token> tokens = lexer.start();
  for (auto token : tokens) {
    std::cout << token.print() << "\n";
  }  // for

  return EXIT_SUCCESS;
}
