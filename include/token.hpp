#ifndef TOKEN_H_
#define TOKEN_H_

#include <optional>
#include <string>

// Type of a C-Moon Token
enum class TokenType {
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
  NUMERIC_LITERAL,
  IF_KEYWORD,
  ELSE_KEYWORD,
  WHILE_KEYWORD,
  EOF_TOKEN,
  ERROR,

};  // TokenType

std::optional<TokenType> keyword_from_string(const std::string& value);
std::optional<TokenType> symbol_from_string(const std::string& value);  // TODO

enum VarType {
  INT,
  DOUBLE,
  UNSIGNED_LONG,
  LONG,
  UNSIGNED_INT,
};

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
      : lexeme(_lexeme), type(_type), pos(_pos), var_type({}) {}  // Constructor

  Token(std::string _lexeme, TokenType _type, Position _pos,
        VarType _var_type) noexcept
      : lexeme(_lexeme),
        type(_type),
        pos(_pos),
        var_type(std::make_optional(_var_type)) {}

  // Pretty printer for Token type
  std::string const print() noexcept;

  // Equality operator overload for testing
  bool operator==(Token const& other) const;

 private:
  std::string lexeme;
  TokenType type;
  Position pos;
  std::optional<VarType> var_type;

};  // Token

#endif  // TOKEN_H_
