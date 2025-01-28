const std = @import("std");

pub const TokenType = enum {
    ILLEGAL,
    EOF,

    // Identifiers & literals
    STRING,

    // Operators
    EQUAL,
    PLUS,
    MINUS,
    STAR,
    SLASH,

    BANG,
    LT,
    GT,
    EQ,
    NEQ,

    // Delimiters
    COMMA,
    SEMI,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    SEMICOLON,

    // Keywords
    IF,
    ELSE,
    RETURN,
    INT,
    VOID,
};

pub const Token = struct {
    kind: TokenType = .ILLEGAL,
    literal: []const u8 = "",

    pub fn init(kind: TokenType, literal: []const u8) Token {
        return Token{
            .kind = kind,
            .literal = literal,
        };
    }
};

pub const Keywords = [_]Token{
    Token.init(.IF, "if"),
    Token.init(.ELSE, "else"),
    Token.init(.RETURN, "return"),
    Token.init(.INT, "int"),
    Token.init(.INT, "void"),
};

pub fn lookupIdentifier(ident: []const u8) TokenType {
    for (Keywords) |keyword| {
        if (std.mem.eql(u8, ident, keyword.literal)) {
            return keyword.kind;
        }
    }

    return .IDENT;
}
