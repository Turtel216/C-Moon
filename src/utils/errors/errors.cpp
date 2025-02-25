#include "errors.h"

namespace cmoon {

// Get the cmoon_error msg
[[nodiscard]] auto inline error::error_msg() noexcept -> std::string const {
  return msg;
}  // error_msg
}  // namespace cmoon
