#include "../include/token.hpp"

#include <optional>
#include <string>
#include <unordered_map>

bool Position::operator==(Position const& rhs) const noexcept {
  return (line == rhs.line && column == rhs.column);
}

// Pretty printer for Token type
std::string const Token::print() noexcept {
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
bool Token::operator==(const Token& other) const {
  return (lexeme == other.lexeme && type == other.type && pos == other.pos);
}

std::optional<TokenType> keyword_from_string(const std::string& value) {
  static const std::unordered_map<std::string, TokenType> tokenMap = {
      // TODO: Complete all keywords
      {"IF", TokenType::IF_KEYWORD},
      {"ELSE", TokenType::ELSE_KEYWORD},
      {"WHILE", TokenType::WHILE_KEYWORD},
      {"RETURN", TokenType::RETURN_KEYWORD},
      {"VOID", TokenType::VOID_KEYWORD},
  };

  auto it = tokenMap.find(value);
  if (it != tokenMap.end()) {
    return it->second;
  }

  // Not found — return as identifier by default
  return {};
}

std::optional<TokenType> symbol_from_string(const std::string& value) {
  static const std::unordered_map<std::string, TokenType> tokenMap = {
      // TODO: Complete all symbols
      {";", TokenType::SEMICOLON},          {"(", TokenType::OPEN_PARENTHESIS},
      {")", TokenType::CLOSED_PARENTHESIS}, {"}", TokenType::OPEN_BRACE},
      {"{", TokenType::OPEN_BRACE},
  };

  auto it = tokenMap.find(value);
  if (it != tokenMap.end()) {
    return it->second;
  }

  // Not found — return as identifier by default
  return {};
}
