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
)

# Library for lexer
add_library(lexer_lib
    src/utils/errors/errors.cpp
    src/lexer/lexer.cpp
    src/lexer/token.cpp
)

# Main executable
add_executable(cm ${SRC_FILES})

# Tests
if (BUILD_TESTING)
    add_executable(lexer_test_success tests/lexer/test_success.cpp)
    target_link_libraries(lexer_test_success lexer_lib)

    add_executable(lexer_test_failure tests/lexer/test_failure.cpp)
    target_link_libraries(lexer_test_failure lexer_lib)

    add_test(NAME lexer_success COMMAND lexer_test_success)
    add_test(NAME lexer_failure COMMAND lexer_test_failure)
endif()

# C++ standard
set_target_properties(cm lexer_lib lexer_test_success lexer_test_failure PROPERTIES
    CXX_STANDARD 23
    CXX_STANDARD_REQUIRED ON
    CXX_EXTENSIONS OFF
)
