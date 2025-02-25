cmake_minimum_required(VERSION 3.30)
project(cmoon)

enable_testing()

add_executable ( cm src/main.cpp src/lexer/lexer.cpp src/lexer/token.cpp)

# Tests
add_executable (lexer_test src tests/lexer/test.cpp src/lexer/lexer.cpp src/lexer/token.cpp)

add_test(lexer lexer_test)
