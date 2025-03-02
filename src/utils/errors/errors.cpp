#include "errors.h"

namespace cmoon {

// Get the cmoon_error msg
[[nodiscard]] auto inline error::error_msg() noexcept -> std::string const {
  return msg;
}  // error_msg

auto LexerError::operator==(LexerError& other) noexcept -> bool const {
  return error_msg() == other.error_msg();
}  // == operator overload

auto ParserError::operator==(ParserError& other) noexcept -> bool const {
  return error_msg() == other.error_msg();
}  // == operator overload
}  // namespace cmoon
