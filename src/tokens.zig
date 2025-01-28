pub const Token = union(enum) {
    Illegal,
    Eof,
    Program,

    // Identifiers & literals
    String,
    Identifier: void,

    // Operators
    Equal,
    Plus,
    Minus,
    Star,
    Slash,

    Bang,
    Lt,
    Gt,
    Eq,
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
