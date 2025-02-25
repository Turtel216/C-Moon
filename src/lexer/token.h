#ifndef TOKEN_H_
#define TOKEN_H_

#include <string>

// Type of a C-Moon Token
enum TokenType {
  INT_KEYWORD,         // Represents the 'int' keyword.
  VOID_KEYWORD,        // Represents the 'void' keyword.
  IDENTIFIER,          // Any valid identifier (e.g., function/variable names).
  OPEN_PARENTHESIS,    // Represents '('.
  CLOSED_PARENTHESIS,  // Represents ')'.
  OPEN_BRACE,          // Represents '{'.
  CLOSED_BRACE,        // Represents '}'.
  CONSTANT,            // Represents numeric constants (e.g., 123).
  SEMICOLON,           // Represents ';'.
  RETURN_KEYWORD,      // Represents the 'return' keyword.
};  // TokenType

// Token recognized by the C-Moon compiler
struct Token {
  std::string lexeme;
  TokenType type;

  Token() = delete;  // Default Constructor

  Token(std::string lexeme, TokenType type) noexcept
      : lexeme(lexeme), type(type) {}  // Constructor
};  // Token

#endif  // TOKEN_H_
