cmake_minimum_required(VERSION 3.30)
project(SinnersC)

# enable_testing()

add_executable ( gng src/main.cpp )

#Tests
# add_executable ( scanner_test test/scanner_test.cpp src/Xanadu.cpp src/Types/Token.cpp src/Types/Literal.cpp src/Scanner/Scanner.cpp src/Parser/Parser.cpp)

# add_test(scanner-test scanner_test)
