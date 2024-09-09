// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file.

use std::fmt;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum TokenType {
    IntKeyword,         // int
    VoidKeyword,        // void
    Identifier(String), // lexeme
    OpenParenthesis,    // (
    CloseParenthesis,   // )
    OpenBrace,          // {
    CloseBrace,         // }
    Constant(isize),    // 123
    Semicolon,          // ;
    ReturnKeyword,      // return
}

// For debuging
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::IntKeyword => write!(f, "IntKeyword"),
            TokenType::VoidKeyword => write!(f, "VoidKeyword"),
            TokenType::Identifier(s) => write!(f, "Identifier '{}'", s),
            TokenType::Constant(v) => write!(f, "Constant '{}'", v),
            TokenType::OpenParenthesis => write!(f, "OpenParenthesis"),
            TokenType::CloseParenthesis => write!(f, "CloseParenthesis"),
            TokenType::OpenBrace => write!(f, "OpenBrace"),
            TokenType::CloseBrace => write!(f, "CloseBrace"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::ReturnKeyword => write!(f, "ReturnKeyword"),
        }
    }
}

// Struct for tokenizing source string
pub struct Tokenizer<'s> {
    tokens: Vec<TokenType>, // Vector holding generated tokens
    source: &'s str,        // Source string to be scanner
    start: usize,           // Start of current lexeme
    current: usize,         // Index of current character
    line: usize,
}

impl<'s> Tokenizer<'s> {
    // Instantiate The Scanner
    pub fn new(source: &'s str) -> Self {
        Self {
            tokens: Vec::new(),
            source,
            start: 0,
            current: 0,
            line: 0,
        }
    }

    // Scan the source string and return a Vector holding the TokenTypes
    pub fn scan_source(&mut self) -> &Vec<TokenType> {
        // Tokenize the source string
        // and add each token to the token vector
        while !self.is_at_end() {
            self.scan_token();
        }

        return &self.tokens;
    }

    // Scan the current selection of the source string and add the poper token to the TokenType
    // Vectror
    fn scan_token(&mut self) -> () {
        // Remove all white space
        self.skip_whitespace();
        // Set start of current lexeme
        self.start = self.current;

        match self.advance() {
            '(' => self.tokens.push(TokenType::OpenParenthesis),
            ')' => self.tokens.push(TokenType::CloseParenthesis),
            '{' => self.tokens.push(TokenType::OpenBrace),
            '}' => self.tokens.push(TokenType::CloseBrace),
            ';' => self.tokens.push(TokenType::Semicolon),
            c => {
                if c.is_numeric() {
                    self.lex_constant();
                    return;
                } else if c.is_alphabetic() {
                    self.lex_identifier();
                    return;
                }

                panic!(
                    "Lexer error: Could'nt recognise character on line {}",
                    self.line
                );
            }
        }
    }

    // Scan for identifier or keyword and add its type to the Token Vector
    fn lex_identifier(&mut self) -> () {
        // Consume all alphanumeric characters
        while self.peek().is_alphabetic() || self.peek().is_numeric() {
            self.advance();
        }

        // Create current lexeme
        let value: &str = self.source[self.start..self.current].into();

        // Check if the token is an identifier
        // or a keyword and add it to the vector
        self.tokens.push(self.match_keyword(value));
    }

    // Check if identifier is a keyword, return its type. If its not a keyword return identifier
    // type
    fn match_keyword(&self, word: &str) -> TokenType {
        match word {
            "int" => TokenType::IntKeyword,
            "void" => TokenType::VoidKeyword,
            "return" => TokenType::ReturnKeyword,
            _ => TokenType::Identifier(self.get_lexeme().to_string()), //TODO get proper lexeme
        }
    }
    // Scan number and add its type to the Token Vector
    fn lex_constant(&mut self) -> () {
        // Consume all numeric characters
        while self.peek().is_numeric() && !self.is_at_end() {
            self.advance();
        }

        let constant = self.get_constant();

        self.tokens.push(TokenType::Constant(constant)); //TODO get constant value
    }

    // Skip all whitespace/comments characters from source string
    fn skip_whitespace(&mut self) -> () {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' if self.peek_next() == '/' => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    // Get the constant value from source string
    fn get_constant(&self) -> isize {
        // Get lexeme
        let constant = self.get_lexeme();
        match constant.parse() {
            Ok(v) => v,
            Err(e) => panic!("Error when trying to get consant from string: {}", e),
        }
    }

    // Get the lexeme value from source string
    fn get_lexeme(&self) -> &str {
        &self.source[self.start..self.current]
    }

    // Get current character. Get \0 if at the end
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self
            .source
            .chars()
            .nth(self.current)
            .unwrap_or_else(|| panic!("Error in peek(). No character at index {}", self.current));
    }

    // Get current char and continue to next character
    fn advance(&mut self) -> char {
        let char = self.peek();
        self.current += 1;
        char
    }

    // Get next character. Get \0 if the next character is at the end
    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self
            .source
            .chars()
            .nth(self.current + 1)
            .unwrap_or_else(|| {
                panic!(
                    "Error in peek_next(). No character at index {}. Current character was {}",
                    self.current,
                    self.source.chars().nth(self.current).unwrap()
                )
            });
    }

    // Check if scanner reached the end of source string
    fn is_at_end(&self) -> bool {
        return self.current == self.source.len();
    }
}

#[cfg(test)]
mod tests {

    use crate::lexer;
    use crate::lexer::TokenType;

    #[test]
    fn test_lexer() {
        // Initialise lexer
        let mut scanner = lexer::Tokenizer::new("void int 123 ( ) { } name return ;");
        // Generate vector of TokenTypes
        let tokens = scanner.scan_source();

        // Vector holding the expected TokenTypes
        let mut expected_tokens = vec![
            TokenType::VoidKeyword,
            TokenType::IntKeyword,
            TokenType::Constant(123),
            TokenType::OpenParenthesis,
            TokenType::CloseParenthesis,
            TokenType::OpenBrace,
            TokenType::CloseBrace,
            TokenType::Identifier("name".to_string()),
            TokenType::ReturnKeyword,
            TokenType::Semicolon,
        ];

        // Make sure both vectors hold the same amount of TokenTypes
        assert!(tokens.len() == expected_tokens.len());

        let mut etoken_iter = expected_tokens.iter_mut(); // Expected Token Iterator

        for token in tokens {
            // Get expected token
            let etoken = match etoken_iter.next() {
                Some(v) => v,
                None => panic!("Couldn't get next token"),
            };

            // Assure that the token generated
            // by the lexer is equal to the expected token
            assert!(token == etoken);
        }
    }
}
