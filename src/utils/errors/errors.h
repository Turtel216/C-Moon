#ifndef ERRORS_H_
#define ERRORS_H_

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
};  // cmoon_error

// Error type for the C-Moon lexer
class lexer_error : error {
 public:
  lexer_error() = delete;  // Default Constructor
  lexer_error(const std::string msg) noexcept : error(msg) {}  // Constructor
};  // lexer_error
}  // namespace cmoon

#endif  // ERRORS_H_
