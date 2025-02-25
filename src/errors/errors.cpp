#include "errors.h"

// Get the cmoon_error msg
[[nodiscard]] auto inline cmoon_error::error_msg() noexcept
    -> std::string const {
  return msg;
}
