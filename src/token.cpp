#include "../include/token.hpp"

#include <string>

bool Position::operator==(Position const& rhs) const noexcept {
  return (line == rhs.line && column == rhs.column);
}

// Pretty printer for Token type
auto Token::print() noexcept -> std::string const {
  std::string type_str;
  switch (type) {
    case TokenType::VOID_KEYWORD:
      type_str = "void";
      break;
    case TokenType::INT_KEYWORD:
      type_str = "int";
      break;
    case TokenType::IDENTIFIER:
      type_str = "identifier";
      break;
    case TokenType::CONSTANT:
      type_str = "Constant";
      break;
    case TokenType::SEMICOLON:
      type_str = "Semicolon";
      break;
    case TokenType::OPEN_BRACE:
      type_str = "Open Brace";
      break;
    case TokenType::CLOSED_BRACE:
      type_str = "Closed Brace";
      break;
    case TokenType::OPEN_PARENTHESIS:
      type_str = "Open Parenthesis";
      break;
    case TokenType::CLOSED_PARENTHESIS:
      type_str = "Closed Parenthesis";
      break;
    case TokenType::RETURN_KEYWORD:
      type_str = "Return";
      break;
    default:
      type_str = "Unreachable Type";
      break;
  }  // switch

  return "Token lexeme: '" + lexeme + "' Token type: " + type_str;
}  // print

// Equality operator overload for testing
auto Token::operator==(const Token& other) const -> bool {
  return (lexeme == other.lexeme && type == other.type && pos == other.pos);
}
