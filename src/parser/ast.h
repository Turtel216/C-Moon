// ast.h - Contains AST node class definitions
#ifndef AST_H
#define AST_H

#include <memory>
#include <string>
#include <utility>

namespace ast {

// Diffrent types of AST nodes
enum Type {
  PROGRAM,
  FUNCTION,
  RETURN,
  CONSTANT,
};  // type

// A struct representing a node in the AST
struct Node {
  std::string value;           // string value of node
  std::unique_ptr<Node> next;  // next node in AST reprentation
  Type node_type;              // type of AST node

  Node(std::string value, Type node_type) noexcept
      : value(std::move(value)), node_type(node_type) {};  // Constructor

  // String reprentation of the Ast node
  auto print() noexcept -> std::string const;
};  // node
}  // namespace ast

#endif  // AST_H
