#ifndef RESULT_H_
#define RESULT_H_

/**
 * @file result.hpp
 * @brief Defines the cmoon::result class template for handling values and
 * errors.
 */

#include <stdexcept>
#include <type_traits>
#include <utility>
#include <variant>

namespace cmoon {

/**
 * @brief A class template to represent either a valid result or an error.
 *
 * The result<T, E> class encapsulates either a value of type T or an error of
 * type E. It provides utility functions to check for and retrieve the value or
 * error, as well as functional programming utilities like transform and
 * and_then.
 *
 * @tparam T The type of the value.
 * @tparam E The type of the error.
 */
template <typename T, typename E>
class result {
 private:
  std::variant<T, E>
      data_;  ///< Stores either a value of type T or an error of type E.

 public:
  /**
   * @brief Constructs a result holding a value.
   * @param value The value to store.
   */
  result(const T& value) : data_(value) {}

  /**
   * @brief Constructs a result holding a value (move constructor).
   * @param value The value to move into the result.
   */
  result(T&& value) : data_(std::move(value)) {}

  /**
   * @brief Constructs a result holding an error.
   * @param error The error to store.
   */
  result(const E& error) : data_(error) {}

  /**
   * @brief Constructs a result holding an error (move constructor).
   * @param error The error to move into the result.
   */
  result(E&& error) : data_(std::move(error)) {}

  /**
   * @brief Checks whether the result contains a valid value.
   * @return True if the result holds a value, false if it holds an error.
   */
  auto has_value() const -> bool { return std::holds_alternative<T>(data_); }

  /**
   * @brief Retrieves the stored value.
   * @throws std::runtime_error if the result holds an error.
   * @return Reference to the stored value.
   */
  auto value() -> T& {
    if (!has_value())
      throw std::runtime_error(
          "Attempted to access value when result holds an error.");
    return std::get<T>(data_);
  }  // value

  /**
   * @brief Retrieves the stored value (const version).
   * @throws std::runtime_error if the result holds an error.
   * @return Const reference to the stored value.
   */
  auto value() const -> const T& {
    if (!has_value())
      throw std::runtime_error(
          "Attempted to access value when result holds an error.");
    return std::get<T>(data_);
  }  // value

  /**
   * @brief Retrieves the stored error.
   * @throws std::runtime_error if the result holds a value.
   * @return Reference to the stored error.
   */
  auto error() -> E& {
    if (has_value())
      throw std::runtime_error(
          "Attempted to access error when result holds a value.");
    return std::get<E>(data_);
  }  // error

  /**
   * @brief Retrieves the stored error (const version).
   * @throws std::runtime_error if the result holds a value.
   * @return Const reference to the stored error.
   */
  auto error() const -> const E& {
    if (has_value())
      throw std::runtime_error(
          "Attempted to access error when result holds a value.");
    return std::get<E>(data_);
  }  // error

  /**
   * @brief Applies a transformation function to the stored value if it exists.
   * @tparam Func The function type to apply.
   * @param func The function to apply to the value.
   * @return A result containing the transformed value or the original error.
   */
  template <typename Func>
  auto transform(Func&& func) -> result<std::invoke_result_t<Func, T>, E> {
    using U = std::invoke_result_t<Func, T>;
    if (has_value()) {
      return result<U, E>(func(value()));
    }  // if
    else {
      return result<U, E>(error());
    }  // else
  }  // transform

  /**
   * @brief Applies a function that returns a result if the stored value exists.
   * @tparam Func The function type to apply.
   * @param func The function to apply to the value.
   * @return The result returned by the function or the original error.
   */
  template <typename Func>
  auto and_then(Func&& func) -> decltype(func(std::declval<T>())) {
    using result_type = decltype(func(std::declval<T>()));
    if (has_value()) {
      return func(value());
    }  // if
    else {
      return result_type(error());
    }  // else
  }  // and_then
};  // result

/**
 * @brief Creates a result holding a value.
 * @tparam T The type of the value.
 * @tparam E The type of the error.
 * @param value The value to store.
 * @return A result instance holding the given value.
 */
template <typename T, typename E>
auto make_result(T value) -> result<T, E> {
  return result<T, E>(std::move(value));
}  // make_result

/**
 * @brief Creates a result holding an error.
 * @tparam T The type of the value.
 * @tparam E The type of the error.
 * @param error The error to store.
 * @return A result instance holding the given error.
 */
template <typename T, typename E>
auto make_error(E error) -> result<T, E> {
  return result<T, E>(std::move(error));
}  // make_error

}  // namespace cmoon

#endif  // RESULT_H_
