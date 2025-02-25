cmake_minimum_required(VERSION 3.30)
project(cmoon LANGUAGES CXX)

# Enable testing
include(CTest)

# Source files
set(SRC_FILES
    src/main.cpp
    src/errors/errors.cpp
    src/lexer/lexer.cpp
    src/lexer/token.cpp
)

# Library for lexer
add_library(lexer_lib
    src/errors/errors.cpp
    src/lexer/lexer.cpp
    src/lexer/token.cpp
)

# Main executable
add_executable(cm ${SRC_FILES})

# Tests
if (BUILD_TESTING)
    add_executable(lexer_test tests/lexer/test.cpp)
    target_link_libraries(lexer_test lexer_lib)

    add_test(NAME lexer COMMAND lexer_test)
endif()

# C++ standard
set_target_properties(cm lexer_lib lexer_test PROPERTIES
    CXX_STANDARD 23
    CXX_STANDARD_REQUIRED ON
    CXX_EXTENSIONS OFF
)
