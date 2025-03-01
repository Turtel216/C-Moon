cmake_minimum_required(VERSION 3.30)
project(cmoon LANGUAGES CXX)

# Enable testing
include(CTest)

# Source files
set(SRC_FILES
    src/main.cpp
    src/utils/errors/errors.cpp
    src/lexer/lexer.cpp
    src/lexer/token.cpp
    src/parser/parser.cpp
    src/parser/ast.cpp
)

# Library for lexer
add_library(lexer_lib
    src/utils/errors/errors.cpp
    src/lexer/lexer.cpp
    src/lexer/token.cpp
)

# Library for parser
add_library(parser_lib
    src/utils/errors/errors.cpp
    src/lexer/lexer.cpp
    src/lexer/token.cpp
    src/parser/ast.cpp
    src/parser/parser.cpp)

# Main executable
add_executable(cm ${SRC_FILES})

# Tests
if (BUILD_TESTING)
    add_executable(lexer_test_success tests/lexer/test_success.cpp)
    target_link_libraries(lexer_test_success lexer_lib)

    add_executable(lexer_test_failure tests/lexer/test_failure.cpp)
    target_link_libraries(lexer_test_failure lexer_lib)

    add_executable(parser_test_success tests/parser/test_success.cpp)
    target_link_libraries(parser_test_success parser_lib)

    add_executable(parser_test_failure tests/parser/test_failure.cpp)
    target_link_libraries(parser_test_failure parser_lib)

    add_test(NAME lexer_success COMMAND lexer_test_success)
    add_test(NAME lexer_failure COMMAND lexer_test_failure)
    add_test(NAME parser_success COMMAND parser_test_success)
    add_test(NAME parser_failure COMMAND parser_test_failure)
endif()

# C++ standard
set_target_properties(cm lexer_lib lexer_test_success lexer_test_failure PROPERTIES
    CXX_STANDARD 23
    CXX_STANDARD_REQUIRED ON
    CXX_EXTENSIONS OFF
)
