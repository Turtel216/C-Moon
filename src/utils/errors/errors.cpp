#include "errors.h"

namespace cmoon {

// Get the cmoon_error msg
[[nodiscard]] auto inline error::error_msg() noexcept -> std::string const {
  return msg;
}  // error_msg

auto lexer_error::operator==(lexer_error& other) noexcept -> bool const {
  return error_msg() == other.error_msg();
}  // == operator overload
}  // namespace cmoon
