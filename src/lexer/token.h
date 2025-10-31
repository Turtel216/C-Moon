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
  EOF_TOKEN,
  ERROR,
};  // TokenType

class Position {
 public:
  int line, column;
  Position(int _line, int _column) noexcept : line(_line), column(_column) {}

  bool operator==(Position const& rhs) const noexcept;
};

// Token recognized by the C-Moon compiler
class Token {
 public:
  Token() = delete;  // Default Constructor

  Token(std::string _lexeme, TokenType _type, Position _pos) noexcept
      : lexeme(_lexeme), type(_type), pos(_pos) {}  // Constructor

  // Pretty printer for Token type
  auto print() noexcept -> std::string const;

  // Equality operator overload for testing
  auto operator==(Token const& other) const -> bool;

 private:
  std::string lexeme;
  TokenType type;
  Position pos;

};  // Token

#endif  // TOKEN_H_
