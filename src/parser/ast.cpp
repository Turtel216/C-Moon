#include "ast.h"

namespace ast {

// Helper function for getting the string representation of
// an AST node type
auto type_to_string(const type& t) noexcept -> std::string;

// String reprentation of the Ast node
auto node::print() noexcept -> std::string const {
  return "Node with Value: " + value +
         "\nand Type: " + type_to_string(node_type);
}  // print

// Helper function for getting the string representation of
// an AST node type
auto type_to_string(const type& t) noexcept -> std::string {
  switch (t) {
    case type::PROGRAM:
      return "Program";
      break;
    case type::FUNCTION:
      return "Function";
      break;
    case type::RETURN:
      return "Return";
      break;
    case type::CONSTANT:
      return "Constant";
      break;
    default:
      return "Unrecognized token";
      break;
  };
}  // type_to_string
}  // namespace ast
