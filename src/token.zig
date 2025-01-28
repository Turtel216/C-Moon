pub const Token = union(enum) {
    Illegal,
    Eof,
    Program,

    // Identifiers & literals
    String: []const u8,
    Identifier: []const u8,

    // Operators
    EqualEqual,
    Equal,
    Plus,
    Minus,
    Star,
    Slash,

    Bang,
    Lt,
    Gt,
    Neq,

    // Delimiters
    Comma,
    Semi,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Semicolon,

    // Keywords
    If,
    Else,
    Return,
    Int: isize,
    Void,
};
