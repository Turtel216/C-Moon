#ifndef ERRORS_H_
#define ERRORS_H_

#include <string>

// Error type for the C-Moon Compiler
class cmoon_error {
 private:
  const std::string msg;

 public:
  cmoon_error() = delete;  // Default Constructor
  cmoon_error(const std::string msg) noexcept : msg(msg) {};  // Constructor

  // Get the cmoon_error msg
  auto error_msg() noexcept -> std::string const;
};  // cmoon_error

// Error type for the C-Moon lexer
class lexer_error : cmoon_error {
 public:
  lexer_error() = delete;  // Default Constructor
  lexer_error(const std::string msg) noexcept
      : cmoon_error(msg) {}  // Constructor
};  // lexer_error

#endif  // ERRORS_H_
