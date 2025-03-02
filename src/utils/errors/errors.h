#ifndef ERRORS_H_
#define ERRORS_H_

#include <stdexcept>
#include <string>

namespace cmoon {
// Error type for the C-Moon Compiler
class error {
 private:
  const std::string msg;

 public:
  error() = delete;                                     // Default Constructor
  error(const std::string msg) noexcept : msg(msg) {};  // Constructor

  // Get the cmoon_error msg
  auto error_msg() noexcept -> std::string const;
};  // Error

// Error type for the C-Moon lexer
class LexerError : error {
 public:
  LexerError() = delete;  // Default Constructor
  LexerError(const std::string msg) noexcept : error(msg) {}  // Constructor

  // overloaded equality operator
  auto operator==(LexerError& other) noexcept -> bool const;
};  // LexerError

// Error type for the C-Moon Parser
class ParserError : error {
 public:
  ParserError() = delete;  // Default Constructor
  ParserError(const std::string msg) noexcept : error(msg) {}  // Constructor

  // overloaded equality operator
  auto operator==(ParserError& other) noexcept -> bool const;
};  // ParserError

// Simple parser exception class
class ParseException : public std::runtime_error {
 public:
  explicit ParseException(const std::string& msg) : std::runtime_error(msg) {}
};  // ParseException
}  // namespace cmoon

#endif  // ERRORS_H_
