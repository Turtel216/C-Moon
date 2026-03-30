//! C lexer implementation

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    //Auto, Unsupported for now
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Int,
    Long,
    //Register, Unsupported for now
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,

    // Identifier and Literals
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,

    // Operators and Punctuatoes
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    EqEq,
    Bang,
    BangEq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    Ampersand,
    AmpAmp,
    Pipe,
    PipePipe,
    Caret,
    Tilde,
    Shl,
    Shr,
    PlusPlus,
    MinusMinus,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    AmpEq,
    PipeEq,
    CaretEq,
    ShlEq,
    ShrEq,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
    Colon,
    Dot,
    Arrow,
    Question,

    Eof,
    Error(LexError),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LexError {
    UnexpectedChar,
    UnterminatedString,
    UnterminatedChar,
    UnterminatedBlockComment,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str,
    pub span: Span,
}

pub struct Lexer<'a> {
    input: &'a str,
    bytes: &'a [u8],
    pos: usize,
    line: usize,
    column: usize,
}
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            bytes: input.as_bytes(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Returns the next character without consuming it
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    /// Returns the character after the next without consuming it
    fn peek_next(&self) -> Option<u8> {
        self.bytes.get(self.pos + 1).copied()
    }

    /// Consumes the next character and advances line/column counters
    fn advance(&mut self) -> Option<u8> {
        if let Some(&c) = self.bytes.get(self.pos) {
            self.pos += 1;
            if c == b'\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(c)
        } else {
            None
        }
    }

    /// Consumes characters while the predicate is true
    fn consume_while<F>(&mut self, mut predicate: F)
    where
        F: FnMut(u8) -> bool,
    {
        while let Some(c) = self.peek() {
            if predicate(c) {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r') => {
                    self.advance();
                }
                Some(b'/') => {
                    match self.peek_next() {
                        Some(b'/') => {
                            // Line comment
                            self.consume_while(|c| c != b'\n');
                        }
                        Some(b'*') => {
                            // Block comment
                            self.advance(); // consume '/'
                            self.advance(); // consume '*'
                            loop {
                                match self.advance() {
                                    Some(b'*') if self.peek() == Some(b'/') => {
                                        self.advance(); // consume '/'
                                        break;
                                    }
                                    Some(_) => continue,
                                    None => break, // Handle unterminated gracefully in parser if needed
                                }
                            }
                        }
                        _ => break,
                    }
                }
                _ => break,
            }
        }
    }

    fn check_keyword(lexeme: &str) -> Option<TokenKind> {
        match lexeme {
            //"auto" => Some(TokenKind::Auto),
            "break" => Some(TokenKind::Break),
            "case" => Some(TokenKind::Case),
            "char" => Some(TokenKind::Char),
            "const" => Some(TokenKind::Const),
            "continue" => Some(TokenKind::Continue),
            "default" => Some(TokenKind::Default),
            "do" => Some(TokenKind::Do),
            "double" => Some(TokenKind::Double),
            "else" => Some(TokenKind::Else),
            "enum" => Some(TokenKind::Enum),
            "extern" => Some(TokenKind::Extern),
            "float" => Some(TokenKind::Float),
            "for" => Some(TokenKind::For),
            "goto" => Some(TokenKind::Goto),
            "if" => Some(TokenKind::If),
            "int" => Some(TokenKind::Int),
            "long" => Some(TokenKind::Long),
            //"register" => Some(TokenKind::Register),
            "return" => Some(TokenKind::Return),
            "short" => Some(TokenKind::Short),
            "signed" => Some(TokenKind::Signed),
            "sizeof" => Some(TokenKind::Sizeof),
            "static" => Some(TokenKind::Static),
            "struct" => Some(TokenKind::Struct),
            "switch" => Some(TokenKind::Switch),
            "typedef" => Some(TokenKind::Typedef),
            "union" => Some(TokenKind::Union),
            "unsigned" => Some(TokenKind::Unsigned),
            "void" => Some(TokenKind::Void),
            "volatile" => Some(TokenKind::Volatile),
            "while" => Some(TokenKind::While),
            _ => None,
        }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace_and_comments();

        let start_pos = self.pos;
        let start_line = self.line;
        let start_col = self.column;

        let Some(c) = self.advance() else {
            return Token {
                kind: TokenKind::Eof,
                lexeme: "",
                span: Span {
                    line: start_line,
                    column: start_col,
                    length: 0,
                },
            };
        };

        let kind = match c {
            // Identifiers and Keywords
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                self.consume_while(|c| c.is_ascii_alphanumeric() || c == b'_');
                let lexeme = &self.input[start_pos..self.pos];
                Self::check_keyword(lexeme).unwrap_or(TokenKind::Identifier)
            }

            // Numeric Literals
            b'0'..=b'9' => {
                let mut is_float = false;
                self.consume_while(|c| c.is_ascii_digit());

                if self.peek() == Some(b'.') {
                    is_float = true;
                    self.advance();
                    self.consume_while(|c| c.is_ascii_digit());
                }

                if is_float {
                    TokenKind::FloatLiteral
                } else {
                    TokenKind::IntegerLiteral
                }
            }

            // String Literals
            b'"' => {
                let mut closed = false;
                while let Some(c) = self.advance() {
                    if c == b'\\' {
                        self.advance(); // Skip escaped char
                    } else if c == b'"' {
                        closed = true;
                        break;
                    }
                }
                if closed {
                    TokenKind::StringLiteral
                } else {
                    TokenKind::Error(LexError::UnterminatedString)
                }
            }

            // Char Literals
            b'\'' => {
                let mut closed = false;
                while let Some(c) = self.advance() {
                    if c == b'\\' {
                        self.advance();
                    } else if c == b'\'' {
                        closed = true;
                        break;
                    }
                }
                if closed {
                    TokenKind::CharLiteral
                } else {
                    TokenKind::Error(LexError::UnterminatedChar)
                }
            }

            // Operators (Applying Maximal Munch Principle)
            b'+' => {
                if self.peek() == Some(b'+') {
                    self.advance();
                    TokenKind::PlusPlus
                } else if self.peek() == Some(b'=') {
                    self.advance();
                    TokenKind::PlusEq
                } else {
                    TokenKind::Plus
                }
            }
            b'-' => {
                if self.peek() == Some(b'-') {
                    self.advance();
                    TokenKind::MinusMinus
                } else if self.peek() == Some(b'>') {
                    self.advance();
                    TokenKind::Arrow
                } else if self.peek() == Some(b'=') {
                    self.advance();
                    TokenKind::MinusEq
                } else {
                    TokenKind::Minus
                }
            }
            b'=' => {
                if self.peek() == Some(b'=') {
                    self.advance();
                    TokenKind::EqEq
                } else {
                    TokenKind::Eq
                }
            }
            b'!' => {
                if self.peek() == Some(b'=') {
                    self.advance();
                    TokenKind::BangEq
                } else {
                    TokenKind::Bang
                }
            }
            b'<' => {
                if self.peek() == Some(b'=') {
                    self.advance();
                    TokenKind::LessEq
                } else if self.peek() == Some(b'<') {
                    self.advance();
                    if self.peek() == Some(b'=') {
                        self.advance();
                        TokenKind::ShlEq
                    } else {
                        TokenKind::Shl
                    }
                } else {
                    TokenKind::Less
                }
            }
            b'>' => {
                if self.peek() == Some(b'=') {
                    self.advance();
                    TokenKind::GreaterEq
                } else if self.peek() == Some(b'>') {
                    self.advance();
                    if self.peek() == Some(b'=') {
                        self.advance();
                        TokenKind::ShrEq
                    } else {
                        TokenKind::Shr
                    }
                } else {
                    TokenKind::Greater
                }
            }

            b'*' => {
                if self.peek() == Some(b'=') {
                    self.advance();
                    TokenKind::StarEq
                } else {
                    TokenKind::Star
                }
            }
            b'/' => {
                if self.peek() == Some(b'=') {
                    self.advance();
                    TokenKind::SlashEq
                } else {
                    TokenKind::Slash
                }
            }
            b'%' => {
                if self.peek() == Some(b'=') {
                    self.advance();
                    TokenKind::PercentEq
                } else {
                    TokenKind::Percent
                }
            }
            b'&' => {
                if self.peek() == Some(b'&') {
                    self.advance();
                    TokenKind::AmpAmp
                } else if self.peek() == Some(b'=') {
                    self.advance();
                    TokenKind::AmpEq
                } else {
                    TokenKind::Ampersand
                }
            }
            b'|' => {
                if self.peek() == Some(b'|') {
                    self.advance();
                    TokenKind::PipePipe
                } else if self.peek() == Some(b'=') {
                    self.advance();
                    TokenKind::PipeEq
                } else {
                    TokenKind::Pipe
                }
            }
            b'^' => {
                if self.peek() == Some(b'=') {
                    self.advance();
                    TokenKind::CaretEq
                } else {
                    TokenKind::Caret
                }
            }
            b'~' => TokenKind::Tilde,
            b'?' => TokenKind::Question,
            b':' => TokenKind::Colon,

            // Basic Punctuation
            b'(' => TokenKind::LParen,
            b')' => TokenKind::RParen,
            b'{' => TokenKind::LBrace,
            b'}' => TokenKind::RBrace,
            b'[' => TokenKind::LBracket,
            b']' => TokenKind::RBracket,
            b';' => TokenKind::Semicolon,
            b',' => TokenKind::Comma,
            b'.' => TokenKind::Dot,

            _ => TokenKind::Error(LexError::UnexpectedChar),
        };

        let length = self.pos - start_pos;
        Token {
            kind,
            lexeme: &self.input[start_pos..self.pos],
            span: Span {
                line: start_line,
                column: start_col,
                length,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to collect all tokens from the input until Eof.
    fn lex_all(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token();
            if token.kind == TokenKind::Eof {
                break;
            }
            tokens.push(token);
        }
        tokens
    }

    #[test]
    fn test_keywords_and_identifiers() {
        let input = "int main auto_var";
        let tokens = lex_all(input);

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::Int);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].lexeme, "main");
        assert_eq!(tokens[2].kind, TokenKind::Identifier);
        assert_eq!(tokens[2].lexeme, "auto_var");
    }

    #[test]
    fn test_numeric_literals() {
        let input = "42 3.14 0";
        let tokens = lex_all(input);

        assert_eq!(tokens[0].kind, TokenKind::IntegerLiteral);
        assert_eq!(tokens[0].lexeme, "42");
        assert_eq!(tokens[1].kind, TokenKind::FloatLiteral);
        assert_eq!(tokens[1].lexeme, "3.14");
        assert_eq!(tokens[2].kind, TokenKind::IntegerLiteral);
        assert_eq!(tokens[2].lexeme, "0");
    }

    #[test]
    fn test_strings_and_chars() {
        let input = r#" "hello, world!" 'c' "#;
        let tokens = lex_all(input);

        assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
        assert_eq!(tokens[0].lexeme, "\"hello, world!\"");
        assert_eq!(tokens[1].kind, TokenKind::CharLiteral);
        assert_eq!(tokens[1].lexeme, "'c'");
    }

    #[test]
    fn test_operators_maximal_munch() {
        // Ensures that ++ is parsed as PlusPlus, not Plus, Plus
        let input = "+ ++ += -> <<=";
        let tokens = lex_all(input);

        assert_eq!(tokens[0].kind, TokenKind::Plus);
        assert_eq!(tokens[1].kind, TokenKind::PlusPlus);
        assert_eq!(tokens[2].kind, TokenKind::PlusEq);
        assert_eq!(tokens[3].kind, TokenKind::Arrow);
        assert_eq!(tokens[4].kind, TokenKind::ShlEq);
    }

    #[test]
    fn test_skipping_comments_and_whitespace() {
        let input = "
            // This is a line comment
            int x = 5; /* 
            Block comment 
            */ 
            return x;
        ";
        let tokens = lex_all(input);

        // Should only see: int, x, =, 5, ;, return, x, ;
        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[0].kind, TokenKind::Int);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[5].kind, TokenKind::Return);
    }

    #[test]
    fn test_line_and_column_tracking() {
        let input = "int a;\n  a = 10;";
        let tokens = lex_all(input);

        // 'int'
        assert_eq!(tokens[0].span.line, 1);
        assert_eq!(tokens[0].span.column, 1);

        // 'a'
        assert_eq!(tokens[1].span.line, 1);
        assert_eq!(tokens[1].span.column, 5);

        // 'a' on the second line
        assert_eq!(tokens[3].span.line, 2);
        assert_eq!(tokens[3].span.column, 3);

        // '='
        assert_eq!(tokens[4].span.line, 2);
        assert_eq!(tokens[4].span.column, 5);
    }

    #[test]
    fn test_unterminated_errors() {
        let input = "\"unterminated string";
        let tokens = lex_all(input);

        assert_eq!(
            tokens[0].kind,
            TokenKind::Error(LexError::UnterminatedString)
        );

        let input2 = "'a";
        let tokens2 = lex_all(input2);
        assert_eq!(
            tokens2[0].kind,
            TokenKind::Error(LexError::UnterminatedChar)
        );
    }
}
