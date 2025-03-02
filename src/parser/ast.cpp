#include "ast.h"

namespace ast {

// Helper function for getting the string representation of
// an AST node type
auto type_to_string(const Type& t) noexcept -> std::string;

// String reprentation of the Ast node
auto Node::print() noexcept -> std::string const {
  return "Node with Value: " + value +
         "\nand Type: " + type_to_string(node_type);
}  // print

// Helper function for getting the string representation of
// an AST node type
auto type_to_string(const Type& t) noexcept -> std::string {
  switch (t) {
    case Type::PROGRAM:
      return "Program";
      break;
    case Type::FUNCTION:
      return "Function";
      break;
    case Type::RETURN:
      return "Return";
      break;
    case Type::CONSTANT:
      return "Constant";
      break;
    default:
      return "Unrecognized token";
      break;
  };
}  // type_to_string
}  // namespace ast
